mod support;

use support::{MockServer, fixture_json, str_at};

/// One start-then-poll session: start returns `start_fixture`, the first poll returns it again,
/// the second poll returns `terminal`.
struct Flow {
    server: MockServer,
    start_path: String,
    status_path: String,
}

impl Flow {
    fn new(start_path: &str, status_prefix: &str, start_fixture: &str, terminal: &str) -> Self {
        let id = str_at(&fixture_json(start_fixture), "id").to_owned();
        let server =
            MockServer::start(&[(201, start_fixture), (200, start_fixture), (200, terminal)]);

        Self {
            server,
            start_path: start_path.to_owned(),
            status_path: format!("{status_prefix}/{id}"),
        }
    }

    fn assert_requests(&self) {
        self.server.assert_requests(&[
            ("POST", &self.start_path),
            ("GET", &self.status_path),
            ("GET", &self.status_path),
        ]);
    }
}

#[cfg(feature = "async")]
mod async_client {
    use super::Flow;
    use crate::support::{self, MockServer, date, fixture_json, instant, poll, str_at};
    use idkollen_client::models::*;
    use idkollen_client::{IdkollenClient, IdkollenClientBuilder, IdkollenError};
    use serde_json::Value;

    fn client(server: &MockServer) -> IdkollenClient {
        IdkollenClientBuilder::new(support::CLIENT_ID, support::CLIENT_SECRET)
            .base_url(server.url())
            .build()
            .unwrap()
    }

    fn error_code(fx: &Value) -> ApiErrorCode {
        serde_json::from_value(fx["error"].clone()).unwrap()
    }

    macro_rules! assert_person {
        ($c:expr, $fx:expr) => {
            assert_eq!($c.id, str_at(&$fx, "id"));
            assert_eq!($c.ref_id.as_deref(), Some(str_at(&$fx, "refId")));
            assert_eq!($c.name, str_at(&$fx, "name"));
            assert_eq!($c.given_name, str_at(&$fx, "givenName"));
            assert_eq!($c.surname, str_at(&$fx, "surname"));
        };
    }

    macro_rules! assert_contact {
        ($c:expr, $fx:expr) => {
            assert_eq!(
                $c.ssn.as_ref().map(|s| s.as_str()),
                Some(str_at(&$fx, "ssn"))
            );
            assert_eq!($c.phone.as_deref(), Some(str_at(&$fx, "phone")));
            assert_eq!(
                $c.email.as_ref().map(|e| e.as_str()),
                Some(str_at(&$fx, "email"))
            );
            assert_eq!($c.address.as_deref(), Some(str_at(&$fx, "address")));
            assert_eq!($c.birth_date, Some(date(&$fx, "birthDate")));
            assert_eq!($c.pid.as_deref(), Some(str_at(&$fx, "pid")));
            assert_eq!($c.bank_id.as_deref(), Some(str_at(&$fx, "bankId")));
        };
    }

    macro_rules! terminal_tests {
        ($module:ident, $dir:literal, $segment:literal, $endpoint:ident, $req:expr, $Status:ident) => {
            mod $module {
                use super::*;

                async fn run(terminal: &str) -> $Status {
                    let flow = Flow::new(
                        concat!("/v3/", $segment, "/auth"),
                        concat!("/v3/", $segment, "/auth"),
                        concat!($dir, "/auth_pending.json"),
                        terminal,
                    );
                    let client = client(&flow.server);
                    let $Status::Pending(started) = client.$endpoint().auth($req).await.unwrap()
                    else {
                        panic!("expected Pending");
                    };
                    let result = client
                        .$endpoint()
                        .wait_for_auth(&started.id, poll())
                        .await
                        .unwrap();

                    flow.assert_requests();

                    result
                }

                #[tokio::test]
                async fn failed() {
                    let terminal = concat!($dir, "/auth_failed.json");
                    let fx = fixture_json(terminal);
                    let $Status::Failed(failed) = run(terminal).await else {
                        panic!("expected Failed");
                    };

                    assert_eq!(failed.id, str_at(&fx, "id"));
                    assert_eq!(failed.ref_id.as_deref(), Some(str_at(&fx, "refId")));
                    assert_eq!(failed.error, error_code(&fx));
                }

                #[tokio::test]
                async fn unknown() {
                    let result = run(concat!($dir, "/auth_unknown.json")).await;

                    assert!(matches!(result, $Status::Unknown), "got {result:?}");
                }
            }
        };
    }

    terminal_tests!(
        bankid_se,
        "bankid_se",
        "bankid-se",
        bankid_se,
        BankIdSeAuthRequest::new(),
        BankIdSeStatus
    );
    terminal_tests!(
        bankid_no,
        "bankid_no",
        "bankid-no",
        bankid_no,
        BankIdNoAuthRequest::new(),
        BankIdNoStatus
    );
    terminal_tests!(
        freja,
        "freja",
        "freja",
        freja,
        FrejaAuthRequest::new(),
        FrejaStatus
    );
    terminal_tests!(
        mitid,
        "mitid",
        "mitid",
        mitid,
        MitIdAuthRequest::new(),
        MitIdStatus
    );
    terminal_tests!(ftn, "ftn", "ftn", ftn, FtnAuthRequest::new(), FtnStatus);
    terminal_tests!(
        vipps,
        "vipps",
        "vipps",
        vipps,
        VippsAuthRequest::new(),
        VippsStatus
    );

    fn assert_bankid_se_completed(c: &BankIdSeCompleted, fx: &Value) {
        assert_person!(c, fx);
        assert_eq!(c.ssn.as_str(), str_at(fx, "ssn"));
        assert_eq!(c.cert_start_date, Some(date(fx, "certStartDate")));
        assert_eq!(c.ocsp_response, str_at(fx, "ocspResponse"));
        assert_eq!(c.risk.as_deref(), Some(str_at(fx, "risk")));

        let device = c.device.as_ref().unwrap();

        assert_eq!(
            device.ip_address.as_deref(),
            Some(str_at(&fx["device"], "ipAddress"))
        );
        assert_eq!(device.uhi.as_deref(), Some(str_at(&fx["device"], "uhi")));

        let step_up = c.step_up.as_ref().unwrap();

        assert_eq!(step_up.mrtd, fx["stepUp"]["mrtd"].as_bool());
        assert_eq!(step_up.face, fx["stepUp"]["face"].as_bool());
        assert_eq!(c.address.as_deref(), Some(str_at(fx, "address")));
    }

    #[tokio::test]
    async fn bankid_se_auth_completed() {
        let pending = fixture_json("bankid_se/auth_pending.json");
        let fx = fixture_json("bankid_se/auth_completed.json");
        let flow = Flow::new(
            "/v3/bankid-se/auth",
            "/v3/bankid-se/auth",
            "bankid_se/auth_pending.json",
            "bankid_se/auth_completed.json",
        );
        let client = client(&flow.server);
        let BankIdSeStatus::Pending(started) = client
            .bankid_se()
            .auth(BankIdSeAuthRequest::new())
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(started.ref_id.as_deref(), Some(str_at(&pending, "refId")));
        assert_eq!(
            started.auto_start_token.as_deref(),
            Some(str_at(&pending, "autoStartToken"))
        );
        assert_eq!(
            started.qr_start_token.as_deref(),
            Some(str_at(&pending, "qrStartToken"))
        );
        assert_eq!(
            started.qr_start_secret.as_deref(),
            Some(str_at(&pending, "qrStartSecret"))
        );
        assert_eq!(
            started.hint_code.as_deref(),
            Some(str_at(&pending, "hintCode"))
        );

        let BankIdSeStatus::Completed(c) = client
            .bankid_se()
            .wait_for_auth(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_bankid_se_completed(&c, &fx);
        assert_eq!(c.authenticated_at, Some(instant(&fx, "authenticatedAt")));
        assert_eq!(c.signed_at, None);
    }

    #[tokio::test]
    async fn bankid_se_sign_completed() {
        let fx = fixture_json("bankid_se/sign_completed.json");
        let flow = Flow::new(
            "/v3/bankid-se/sign",
            "/v3/bankid-se/sign",
            "bankid_se/auth_pending.json",
            "bankid_se/sign_completed.json",
        );
        let client = client(&flow.server);
        let BankIdSeStatus::Pending(started) = client
            .bankid_se()
            .sign(BankIdSeSignRequest::new("Sign this"))
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };
        let BankIdSeStatus::Completed(c) = client
            .bankid_se()
            .wait_for_sign(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_bankid_se_completed(&c, &fx);
        assert_eq!(c.signed_at, Some(instant(&fx, "signedAt")));
        assert_eq!(c.authenticated_at, None);
    }

    #[tokio::test]
    async fn bankid_se_phone_auth_completed() {
        let fx = fixture_json("bankid_se/auth_completed.json");
        let flow = Flow::new(
            "/v3/bankid-se/phone/auth",
            "/v3/bankid-se/auth",
            "bankid_se/phone_auth_pending.json",
            "bankid_se/auth_completed.json",
        );
        let client = client(&flow.server);
        let ssn = Pno::parse(str_at(&fx, "ssn")).unwrap();
        let BankIdSeStatus::Pending(started) = client
            .bankid_se()
            .phone_auth(BankIdSePhoneAuthRequest::new(ssn, CallInitiator::Rp))
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(started.auto_start_token, None);
        assert_eq!(started.qr_start_token, None);
        assert_eq!(started.qr_start_secret, None);
        assert_eq!(started.hint_code, None);

        let BankIdSeStatus::Completed(c) = client
            .bankid_se()
            .wait_for_auth(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_bankid_se_completed(&c, &fx);
    }

    #[tokio::test]
    async fn bankid_se_verify() {
        let fx = fixture_json("bankid_se/verify.json");
        let server = MockServer::start(&[(200, "bankid_se/verify.json")]);
        let client = client(&server);
        let result = client
            .bankid_se()
            .verify(BankIdSeVerifyRequest::new("bankid.qr-code"))
            .await
            .unwrap();

        server.assert_requests(&[("POST", "/v3/bankid-se/verify")]);
        assert_eq!(
            serde_json::from_slice::<Value>(&server.requests()[0].body).unwrap(),
            serde_json::json!({ "qrCode": "bankid.qr-code" })
        );
        assert_eq!(result.ssn.as_str(), str_at(&fx, "ssn"));
        assert_eq!(result.name, str_at(&fx, "name"));
        assert_eq!(result.given_name, str_at(&fx, "givenName"));
        assert_eq!(result.surname, str_at(&fx, "surname"));
        assert_eq!(result.age.map(u64::from), fx["age"].as_u64());
        assert_eq!(result.verified_at, Some(date(&fx, "verifiedAt")));
    }

    #[tokio::test]
    async fn bankid_no_auth_completed() {
        let pending = fixture_json("bankid_no/auth_pending.json");
        let fx = fixture_json("bankid_no/auth_completed.json");
        let flow = Flow::new(
            "/v3/bankid-no/auth",
            "/v3/bankid-no/auth",
            "bankid_no/auth_pending.json",
            "bankid_no/auth_completed.json",
        );
        let client = client(&flow.server);
        let BankIdNoStatus::Pending(started) = client
            .bankid_no()
            .auth(BankIdNoAuthRequest::new())
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(started.url.as_deref(), Some(str_at(&pending, "url")));
        assert_eq!(
            started.binding_message.as_deref(),
            Some(str_at(&pending, "bindingMessage"))
        );

        let BankIdNoStatus::Completed(c) = client
            .bankid_no()
            .wait_for_auth(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_contact!(c, fx);
        assert_eq!(c.authenticated_at, Some(instant(&fx, "authenticatedAt")));
    }

    #[tokio::test]
    async fn bankid_no_sign_completed() {
        let fx = fixture_json("bankid_no/sign_completed.json");
        let flow = Flow::new(
            "/v3/bankid-no/sign",
            "/v3/bankid-no/sign",
            "bankid_no/auth_pending.json",
            "bankid_no/sign_completed.json",
        );
        let client = client(&flow.server);
        let BankIdNoStatus::Pending(started) = client
            .bankid_no()
            .sign(BankIdNoSignRequest::new())
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };
        let BankIdNoStatus::Completed(c) = client
            .bankid_no()
            .wait_for_sign(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_contact!(c, fx);
        assert_eq!(c.signed_at, Some(instant(&fx, "signedAt")));
        assert_eq!(
            c.sign_result.unwrap().hash,
            str_at(&fx["signResult"], "hash")
        );

        let documents = c.signed_documents.unwrap();

        assert_eq!(documents[0].id, str_at(&fx["signedDocuments"][0], "id"));
        assert_eq!(documents[0].hash, str_at(&fx["signedDocuments"][0], "hash"));
    }

    #[tokio::test]
    async fn freja_auth_completed() {
        let pending = fixture_json("freja/auth_pending.json");
        let fx = fixture_json("freja/auth_completed.json");
        let flow = Flow::new(
            "/v3/freja/auth",
            "/v3/freja/auth",
            "freja/auth_pending.json",
            "freja/auth_completed.json",
        );
        let client = client(&flow.server);
        let FrejaStatus::Pending(started) =
            client.freja().auth(FrejaAuthRequest::new()).await.unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(
            started.auto_start_token.as_deref(),
            Some(str_at(&pending, "autoStartToken"))
        );
        assert_eq!(started.qr_data.as_deref(), Some(str_at(&pending, "qrData")));

        let FrejaStatus::Completed(c) = client
            .freja()
            .wait_for_auth(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_eq!(c.ssn, str_at(&fx, "ssn"));
        assert_eq!(
            c.country,
            serde_json::from_value::<Country>(fx["country"].clone()).unwrap()
        );
        assert_eq!(c.address.as_deref(), Some(str_at(&fx, "address")));
        assert_eq!(c.authenticated_at, Some(instant(&fx, "authenticatedAt")));
    }

    #[tokio::test]
    async fn freja_sign_completed() {
        let fx = fixture_json("freja/sign_completed.json");
        let flow = Flow::new(
            "/v3/freja/sign",
            "/v3/freja/sign",
            "freja/auth_pending.json",
            "freja/sign_completed.json",
        );
        let client = client(&flow.server);
        let FrejaStatus::Pending(started) = client
            .freja()
            .sign(FrejaSignRequest::new("Sign this"))
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };
        let FrejaStatus::Completed(c) = client
            .freja()
            .wait_for_sign(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_eq!(c.signed_at, Some(instant(&fx, "signedAt")));
    }

    #[tokio::test]
    async fn mitid_auth_completed() {
        let pending = fixture_json("mitid/auth_pending.json");
        let fx = fixture_json("mitid/auth_completed.json");
        let flow = Flow::new(
            "/v3/mitid/auth",
            "/v3/mitid/auth",
            "mitid/auth_pending.json",
            "mitid/auth_completed.json",
        );
        let client = client(&flow.server);
        let MitIdStatus::Pending(started) =
            client.mitid().auth(MitIdAuthRequest::new()).await.unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(started.url.as_deref(), Some(str_at(&pending, "url")));

        let MitIdStatus::Completed(c) = client
            .mitid()
            .wait_for_auth(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_contact!(c, fx);
        assert_eq!(c.authenticated_at, Some(instant(&fx, "authenticatedAt")));
    }

    #[tokio::test]
    async fn mitid_sign_completed() {
        let fx = fixture_json("mitid/sign_completed.json");
        let flow = Flow::new(
            "/v3/mitid/sign",
            "/v3/mitid/sign",
            "mitid/auth_pending.json",
            "mitid/sign_completed.json",
        );
        let client = client(&flow.server);
        let MitIdStatus::Pending(started) = client
            .mitid()
            .sign(MitIdSignRequest::new("Sign this"))
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };
        let MitIdStatus::Completed(c) = client
            .mitid()
            .wait_for_sign(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_eq!(c.signed_at, Some(instant(&fx, "signedAt")));
        assert_eq!(
            c.sign_result.unwrap().checksum,
            str_at(&fx["signResult"], "checksum")
        );
    }

    #[tokio::test]
    async fn ftn_auth_completed() {
        let pending = fixture_json("ftn/auth_pending.json");
        let fx = fixture_json("ftn/auth_completed.json");
        let flow = Flow::new(
            "/v3/ftn/auth",
            "/v3/ftn/auth",
            "ftn/auth_pending.json",
            "ftn/auth_completed.json",
        );
        let client = client(&flow.server);
        let FtnStatus::Pending(started) = client.ftn().auth(FtnAuthRequest::new()).await.unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(started.url, str_at(&pending, "url"));

        let FtnStatus::Completed(c) = client
            .ftn()
            .wait_for_auth(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_contact!(c, fx);
        assert_eq!(c.bank_name.as_deref(), Some(str_at(&fx, "bankName")));
        assert_eq!(c.authenticated_at, Some(instant(&fx, "authenticatedAt")));
    }

    #[tokio::test]
    async fn vipps_auth_completed() {
        let pending = fixture_json("vipps/auth_pending.json");
        let fx = fixture_json("vipps/auth_completed.json");
        let flow = Flow::new(
            "/v3/vipps/auth",
            "/v3/vipps/auth",
            "vipps/auth_pending.json",
            "vipps/auth_completed.json",
        );
        let client = client(&flow.server);
        let VippsStatus::Pending(started) =
            client.vipps().auth(VippsAuthRequest::new()).await.unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(started.url.as_deref(), Some(str_at(&pending, "url")));

        let VippsStatus::Completed(c) = client
            .vipps()
            .wait_for_auth(&started.id, poll())
            .await
            .unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_person!(c, fx);
        assert_contact!(c, fx);
        assert_eq!(c.authenticated_at, Some(instant(&fx, "authenticatedAt")));
    }

    async fn age_verification(terminal: &str) -> AgeVerificationStatus {
        let pending = fixture_json("age_verification/pending.json");
        let flow = Flow::new(
            "/v3/bankid-se/age-verification",
            "/v3/bankid-se/age-verification",
            "age_verification/pending.json",
            terminal,
        );
        let client = client(&flow.server);
        let AgeVerificationStatus::Pending(started) = client
            .bankid_se()
            .age_verification(AgeVerificationRequest::new())
            .await
            .unwrap()
        else {
            panic!("expected Pending");
        };

        assert_eq!(started.url.as_deref(), Some(str_at(&pending, "url")));
        assert_eq!(started.min_age.map(u64::from), pending["minAge"].as_u64());
        assert_eq!(started.max_age, None);

        let result = client
            .bankid_se()
            .wait_for_age_verification(&started.id, poll())
            .await
            .unwrap();

        flow.assert_requests();

        result
    }

    #[tokio::test]
    async fn age_verification_completed() {
        let fx = fixture_json("age_verification/completed.json");
        let AgeVerificationStatus::Completed(c) =
            age_verification("age_verification/completed.json").await
        else {
            panic!("expected Completed");
        };

        assert_eq!(c.id, str_at(&fx, "id"));
        assert_eq!(Some(c.age_verified), fx["ageVerified"].as_bool());
    }

    #[tokio::test]
    async fn age_verification_failed() {
        let fx = fixture_json("age_verification/failed.json");
        let AgeVerificationStatus::Failed(failed) =
            age_verification("age_verification/failed.json").await
        else {
            panic!("expected Failed");
        };

        assert_eq!(failed.error, error_code(&fx));
    }

    #[tokio::test]
    async fn age_verification_unknown() {
        let result = age_verification("age_verification/unknown.json").await;

        assert!(
            matches!(result, AgeVerificationStatus::Unknown),
            "got {result:?}"
        );
    }

    #[tokio::test]
    async fn document_upload() {
        let fx = fixture_json("document/upload.json");
        let server = MockServer::start(&[(201, "document/upload.json")]);
        let client = client(&server);
        let result = client
            .document()
            .upload(b"%PDF-1.7".to_vec(), "contract.pdf", "application/pdf")
            .await
            .unwrap();

        server.assert_requests(&[("POST", "/v3/document")]);
        assert!(
            server.requests()[0]
                .content_type
                .as_deref()
                .is_some_and(|c| c.starts_with("multipart/form-data"))
        );
        assert_eq!(result.id, str_at(&fx, "id"));
        assert_eq!(result.hash, str_at(&fx, "hash"));
    }

    #[tokio::test]
    async fn api_error() {
        let fx = fixture_json("error.json");
        let server = MockServer::start(&[(400, "error.json")]);
        let client = client(&server);
        let result = client.ftn().auth(FtnAuthRequest::new()).await;
        let Err(IdkollenError::Api { status, message }) = result else {
            panic!("expected Api error, got {result:?}");
        };

        assert_eq!(status, 400);
        assert_eq!(message, str_at(&fx, "message"));
    }
}

#[cfg(feature = "blocking")]
mod blocking_client {
    use super::Flow;
    use crate::support::{self, fixture_json, instant, poll, str_at};
    use idkollen_client::IdkollenClientBuilder;
    use idkollen_client::models::*;

    #[test]
    fn ftn_auth_completed() {
        let fx = fixture_json("ftn/auth_completed.json");
        let flow = Flow::new(
            "/v3/ftn/auth",
            "/v3/ftn/auth",
            "ftn/auth_pending.json",
            "ftn/auth_completed.json",
        );
        let client = IdkollenClientBuilder::new(support::CLIENT_ID, support::CLIENT_SECRET)
            .base_url(flow.server.url())
            .build_blocking()
            .unwrap();
        let FtnStatus::Pending(started) = client.ftn().auth(FtnAuthRequest::new()).unwrap() else {
            panic!("expected Pending");
        };
        let FtnStatus::Completed(c) = client.ftn().wait_for_auth(&started.id, poll()).unwrap()
        else {
            panic!("expected Completed");
        };

        flow.assert_requests();
        assert_eq!(c.id, str_at(&fx, "id"));
        assert_eq!(c.ssn.as_ref().map(|s| s.as_str()), Some(str_at(&fx, "ssn")));
        assert_eq!(c.authenticated_at, Some(instant(&fx, "authenticatedAt")));
    }
}
