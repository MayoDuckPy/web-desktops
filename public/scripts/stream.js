let conn = null;
let sdpEvent = null;
let iceEvent = null;

const servers = {
  iceServers: [
    {
      urls: ['stun:stun1.l.google.com:19302', 'stun:stun2.l.google.com:19302'],
    },
  ],
  iceCandidatePoolSize: 10,
};

async function startCapture() {
  let capture = null;
  const displayMediaOptions = {
    video: {
      displaySurface: "monitor",
    },
    audio: true,
  };

  try {
    capture = await navigator.mediaDevices.getDisplayMedia(displayMediaOptions);
    // videoElem.srcObject = await navigator.mediaDevices.getDisplayMedia(displayMediaOptions);
  } catch (err) {
    console.log(`Error: ${err}`);
  }

  return capture;
}

function stopCapture(mediaStream) {
  let tracks = mediaStream.getTracks();

  tracks.forEach((track) => track.stop());
  videoElem.srcObject = null;
}

export async function createServer() {
  let mediaStream = await startCapture();

  conn = new RTCPeerConnection(servers);
  mediaStream.getTracks().forEach((track) => conn.addTrack(track));

  // Trickle ICE candidates to client
  conn.onicecandidate = async event => {
    if (event.candidate == null) { return; };

    await fetch('/api/answer/ice', {
      method: 'POST',
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(event.candidate)
    });
  };

  // Receive client ICE candidates
  iceEvent = new EventSource('/api/offer/ice');
  iceEvent.onmessage = event => {
    let candidate = new RTCIceCandidate(JSON.parse(event.data));
    conn.addIceCandidate(candidate);
  };

  // Setup listener to receive offer
  sdpEvent = new EventSource('/api/offer');
  sdpEvent.onmessage = async event => {
    let offer = JSON.parse(event.data);
    await conn.setRemoteDescription(offer);

    // Create and signal answer
    let answer = await conn.createAnswer();
    await conn.setLocalDescription(answer);
    await fetch('/api/answer', {
        method: 'POST',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(answer)
    });
  };
}

export async function connectClient() {
  conn = new RTCPeerConnection(servers);
  let mediaStream = new MediaStream();

  // Setup video player to use video stream once connected
  let video = document.querySelector('video');
  video.srcObject = mediaStream;
  conn.ontrack = event => {
    if (event.type == 'track')
      mediaStream.addTrack(event.track);
  };

  // Trickle ICE candidates to server
  conn.onicecandidate = async event => {
    if (event.candidate == null) { return; };

    await fetch('/api/offer/ice', {
      method: 'POST',
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(event.candidate)
    });
  };

  // Receive server ice candidates
  iceEvent = new EventSource('/api/answer/ice');
  iceEvent.onmessage = event => {
    let candidate = new RTCIceCandidate(JSON.parse(event.data));
    conn.addIceCandidate(candidate);
  };

  // Setup listener to receive answer
  sdpEvent = new EventSource('/api/answer');
  sdpEvent.onmessage = async event => {
    let answer = JSON.parse(event.data);
    await conn.setRemoteDescription(answer);
  };

  // Create and signal offer
  const offerOptions = {
    'offerToReceiveAudio': true,
    'offerToReceiveVideo': true
  };
  let offer = await conn.createOffer(offerOptions);

  await conn.setLocalDescription(offer);
  await fetch('/api/offer', {
      method: 'POST',
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(offer)
  });
}

export function closeConnection() {
  let video = document.querySelector('video');
  if (video.srcObject)
    video.srcObject = null;

  if (sdpEvent) {
    sdpEvent.onmessage = null;
    sdpEvent.close();
    sdpEvent = null;
  };

  if (iceEvent) {
    iceEvent.onmessage = null;
    iceEvent.close();
    iceEvent = null;
  };

  if (conn) {
    conn.onicecandidate = null;
    conn.close();
    conn = null;
  };
}
