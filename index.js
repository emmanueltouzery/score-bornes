// started from https://codepen.io/chrisbeast/pen/ebYwpX
const controls = document.querySelector(".controls");
const cameraOptions = document.querySelector(".video-options>select");
const video = document.querySelector("video");
const canvas = document.querySelector("canvas");
const screenshotImage = document.querySelector("img");
const buttons = [...controls.querySelectorAll("button")];
let streamStarted = false;
const computeCanvas = document.createElement("canvas");
const WIDTH = 1280;
const HEIGHT = 720;

computeCanvas.width = WIDTH;
computeCanvas.height = HEIGHT;
const ctx = computeCanvas.getContext("2d");

const [play, pause, screenshot] = buttons;

const constraints = {
  video: {
    width: {
      min: WIDTH,
      ideal: WIDTH,
      max: WIDTH
    },
    height: {
      min: HEIGHT,
      ideal: HEIGHT,
      max: HEIGHT
    }
  }
};

cameraOptions.onchange = () => {
  const updatedConstraints = {
    ...constraints,
    deviceId: {
      exact: cameraOptions.value
    }
  };

  startStream(updatedConstraints);
};

play.onclick = () => {
  if (streamStarted) {
    video.play();
    play.classList.add("d-none");
    pause.classList.remove("d-none");
    return;
  }
  if ("mediaDevices" in navigator && navigator.mediaDevices.getUserMedia) {
    const updatedConstraints = {
      ...constraints,
      deviceId: {
        exact: cameraOptions.value
      }
    };
    startStream(updatedConstraints);
  }
  setInterval(findCards, 200);
};

const findCards = () => {
  console.log("got cards");
  ctx.drawImage(video, 0, 0);
  const data = ctx.getImageData(0, 0, WIDTH, HEIGHT).data;
  const bluePixels = [];
  const redPixels = [];
  const blackPixels = [];
  let offset = 0;
  for (let x = 0; x < WIDTH; x++) {
    for (let y = 0; y < HEIGHT; y++) {
      const red = data[offset];
      const green = data[offset + 1];
      const blue = data[offset + 2];
      if (red < 150 && red + 4 < green && green + 4 < blue) {
        bluePixels.push([x, y]);
      }
      offset += 4;
    }
    offset += 4;
  }
  console.log("Found " + bluePixels.length + " blue pixels");
};

// brute force
const pixelsFindContiguousAreas = pixels => {};

const pauseStream = () => {
  video.pause();
  play.classList.remove("d-none");
  pause.classList.add("d-none");
};

const doScreenshot = () => {
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  canvas.getContext("2d").drawImage(video, 0, 0);
  screenshotImage.src = canvas.toDataURL("image/webp");
  screenshotImage.classList.remove("d-none");
};

pause.onclick = pauseStream;
screenshot.onclick = doScreenshot;

const startStream = async constraints => {
  const stream = await navigator.mediaDevices.getUserMedia(constraints);
  handleStream(stream);
};

const handleStream = stream => {
  video.srcObject = stream;
  play.classList.add("d-none");
  pause.classList.remove("d-none");
  screenshot.classList.remove("d-none");
};

const getCameraSelection = async () => {
  const devices = await navigator.mediaDevices.enumerateDevices();
  const videoDevices = devices.filter(device => device.kind === "videoinput");
  const options = videoDevices.map(videoDevice => {
    return `<option value="${videoDevice.deviceId}">${videoDevice.label}</option>`;
  });
  cameraOptions.innerHTML = options.join("");
};

getCameraSelection();
