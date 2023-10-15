use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Offer {
    r#type: String,
    sdp: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Answer {
    r#type: String,
    sdp: String,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
struct IceCandidate {
    address: Option<String>,
    component: Option<String>,
    foundation: Option<String>,
    port: Option<usize>,
    priority: Option<usize>,
    protocol: Option<String>,
    relatedAddress: Option<String>,
    relatedPort: Option<usize>,
    candidate: String,
    sdpMid: String,
    sdpMLineIndex: usize,
    tcpType: Option<String>,
    r#type: Option<String>,
    usernameFragment: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            Json,
            Router,
            extract::State,
            response::sse::{Event, KeepAlive, Sse},
            routing::post,
        };
        use futures::stream::Stream;
        use std::sync::Arc;
        use tokio::sync::broadcast;
        use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

        #[derive(Clone)]
        struct RTCSessionManager {
            pub offer: Arc<broadcast::Sender<Event>>,
            pub answer: Arc<broadcast::Sender<Event>>,
            pub offer_candidates: Arc<broadcast::Sender<Event>>,
            pub answer_candidates: Arc<broadcast::Sender<Event>>,
        }

        pub fn api_routes() -> Router {
            let rtc_session = RTCSessionManager {
                // NOTE: Messages are missed if channel size too small
                offer: Arc::new(broadcast::channel(1).0),
                answer: Arc::new(broadcast::channel(1).0),
                offer_candidates: Arc::new(broadcast::channel(10).0),
                answer_candidates: Arc::new(broadcast::channel(10).0),
            };

            Router::new()
                .route("/offer", post(get_offer).get(send_offer))
                .route("/answer", post(get_answer).get(send_answer))
                .route("/offer/ice", post(get_offer_candidates).get(send_offer_candidates))
                .route("/answer/ice", post(get_answer_candidates).get(send_answer_candidates))
                .with_state(rtc_session)
        }

        async fn send_offer(State(session): State<RTCSessionManager>) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
            let stream = BroadcastStream::new(session.offer.subscribe());

            Sse::new(stream).keep_alive(KeepAlive::default())
        }

        #[axum::debug_handler]
        async fn get_offer(State(session): State<RTCSessionManager>, Json(offer): Json<Offer>) {
            // For now, use the latest offer
            let event = Event::default();
            match event.json_data(offer) {
                Err(_) => println!("Bad offer received"),
                Ok(offer) => { session.offer.send(offer).unwrap(); },
            };
        }

        #[axum::debug_handler]
        async fn send_answer(State(session): State<RTCSessionManager>) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
            let stream = BroadcastStream::new(session.answer.subscribe());

            Sse::new(stream).keep_alive(KeepAlive::default())
        }

        async fn get_answer(State(session): State<RTCSessionManager>, Json(answer): Json<Answer>) {
            // For now, use the latest answer
            let event = Event::default();
            match event.json_data(answer) {
                Err(_) => println!("Bad answer received"),
                Ok(answer) => { session.answer.send(answer).unwrap(); },
            };
        }

        async fn send_offer_candidates(State(session): State<RTCSessionManager>) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
            let stream = BroadcastStream::new(session.offer_candidates.subscribe());

            Sse::new(stream).keep_alive(KeepAlive::default())
        }

        #[axum::debug_handler]
        async fn get_offer_candidates(State(session): State<RTCSessionManager>, Json(candidate): Json<IceCandidate>) {
            let event = Event::default();
            match event.json_data(candidate) {
                Err(_) => println!("Bad offer candidate received"),
                Ok(candidate) => { session.offer_candidates.send(candidate).unwrap(); },
            };
        }

        #[axum::debug_handler]
        async fn send_answer_candidates(State(session): State<RTCSessionManager>) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
            let stream = BroadcastStream::new(session.answer_candidates.subscribe());

            Sse::new(stream).keep_alive(KeepAlive::default())
        }

        async fn get_answer_candidates(State(session): State<RTCSessionManager>, Json(candidate): Json<IceCandidate>) {
            let event = Event::default();
            match event.json_data(candidate) {
                Err(_) => println!("Bad answer candidate received"),
                Ok(candidate) => { session.answer_candidates.send(candidate).unwrap(); },
            };
        }
    }
}
