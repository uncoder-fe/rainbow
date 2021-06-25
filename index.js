const width = 350;
const height = 350;
const colors = [
  [255, 0, 0],
  [255, 255, 0],
  [0, 255, 0],
  [0, 255, 255],
  [0, 0, 255],
  [255, 0, 255],
  [255, 0, 0],
  [255, 255, 0],
].reverse();

function demo1() {
  function getWidthRange(start, width, canvasWidth, lineNumber) {
    const startIndex = start + lineNumber * canvasWidth;
    const endIndex = startIndex + width;
    return [startIndex, endIndex - 1];
  }
  function updateRange(range, data, index) {
    // console.log(range);
    const redIsEqual = colors[index + 1][0] === colors[index][0];
    const red = !redIsEqual
      ? colors[index + 1][0] - colors[index][0]
      : colors[index][0];
    const greenIsEqual = colors[index + 1][1] === colors[index][1];
    const green = !greenIsEqual
      ? colors[index + 1][1] - colors[index][1]
      : colors[index][1];
    const blueIsEqual = colors[index + 1][2] === colors[index][2];
    const blue = !blueIsEqual
      ? colors[index + 1][2] - colors[index][2]
      : colors[index][2];
    // console.log(range);
    for (let j = range[0]; j <= range[1]; j++) {
      const p = (j - range[0]) / 50;
      if (data[j * 4 + 3] > 0) {
        data[j * 4] = redIsEqual ? red : red >= 0 ? red * p : 255 + red * p;
        data[j * 4 + 1] = greenIsEqual
          ? green
          : green >= 0
          ? green * p
          : 255 + green * p;
        data[j * 4 + 2] = blueIsEqual
          ? blue
          : blue >= 0
          ? blue * p
          : 255 + blue * p;
      }
    }
  }
  const canvas = document.querySelector("#playground");
  const ctx = canvas.getContext("2d");
  ctx.font = "50px any";
  ctx.fillText("薇薇安，好酷。", 0, 200);
  const imageData = ctx.getImageData(0, 0, width, height);
  const { data } = imageData;
  // 染色范围，切7份
  for (let i = 0; i < height; i++) {
    // let i =349;
    for (let j = 0; j < 7; j++) {
      const range = getWidthRange(j == 0 ? 0 : 50 * j, 50, width, i);
      updateRange(range, data, j);
    }
  }
  ctx.putImageData(imageData, 0, 0);
}
function demo2() {
  const canvas = document.querySelector("#playground2");
  const ctx = canvas.getContext("2d");
  // 画布渐变
  const gradient = ctx.createLinearGradient(0, 0, width, 0);
  for (let i = 0; i <= 7; i++) {
    gradient.addColorStop(
      i / 7,
      `rgb(${colors[i][0]},${colors[i][1]},${colors[i][2]})`
    );
  }

  ctx.save();
  ctx.font = "50px any";
  ctx.fillStyle = gradient;
  ctx.fillText("薇薇安，好酷。", 0, 200);
  ctx.restore();
}
function demo4(wasm) {
  const canvas = document.querySelector("#playground3");
  const ctx = canvas.getContext("2d");
  ctx.font = "50px any";
  ctx.fillText("薇薇安，好酷。", 0, 200);
  const imageData = ctx.getImageData(0, 0, width, height);
  const { data } = imageData;
  wasm.draw(ctx, 350, 350, data);
}

window.onload = () => {
  // console.log("你好世界");
  demo1();
  demo2();

  // wasm测试
  const rust = import("./pkg");
  rust
    .then((wasm) => {
      console.log("加载rust模块成功");
      // wasm.greet("World!");
      demo4(wasm);
    })
    .catch(console.error);
};
// feat/1.0.0