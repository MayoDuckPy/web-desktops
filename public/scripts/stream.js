export async function startCapture() {
  // let capture = null;
  let videoElem = document.querySelector('video');
  const displayMediaOptions = {
    video: {
      displaySurface: "monitor",
    },
    audio: true,
  };

  try {
    // capture = await navigator.mediaDevices.getDisplayMedia(displayMediaOptions);
    videoElem.srcObject = await navigator.mediaDevices.getDisplayMedia(displayMediaOptions);
  } catch (err) {
    console.log(`Error: ${err}`);
  }

  // return capture;
}

export async function stopCapture() {
  let videoElem = document.querySelector('video');
  let tracks = videoElem.srcObject.getTracks();

  tracks.forEach((track) => track.stop());
  videoElem.srcObject = null;
}
