function rainbowColor(angle) {
  let red, green, blue;
  if (angle < 600) {
    red = 255;
    green = Math.round(angle * 4.25 - 0.01);
    blue = 0;
  } else if (angle < 1200) {
    red = Math.round((1200 - angle) * 4.25 - 0.01);
    green = 255;
    blue = 0;
  } else if (angle < 1800) {
    red = 0;
    green = 255;
    blue = Math.round((angle - 1200) * 4.25 - 0.01);
  } else if (angle < 2400) {
    red = 0;
    green = Math.round((2400 - angle) * 4.25 - 0.01);
    blue = 255;
  } else if (angle < 3000) {
    red = Math.round((angle - 2400) * 4.25 - 0.01);
    green = 0;
    blue = 255;
  } else {
    red = 255;
    green = 0;
    blue = Math.round((360 - angle) * 4.25 - 0.01);
  }
  return [red, green, blue];
}
function HSVtoRGB(h, s, v) {
  var r, g, b, i, f, p, q, t;
  if (arguments.length === 1) {
    (s = h.s), (v = h.v), (h = h.h);
  }
  i = Math.floor(h * 6);
  f = h * 6 - i;
  p = v * (1 - s);
  q = v * (1 - f * s);
  t = v * (1 - (1 - f) * s);
  switch (i % 6) {
    case 0:
      (r = v), (g = t), (b = p);
      break;
    case 1:
      (r = q), (g = v), (b = p);
      break;
    case 2:
      (r = p), (g = v), (b = t);
      break;
    case 3:
      (r = p), (g = q), (b = v);
      break;
    case 4:
      (r = t), (g = p), (b = v);
      break;
    case 5:
      (r = v), (g = p), (b = q);
      break;
  }
  return [Math.round(r * 255), , Math.round(g * 255), , Math.round(b * 255)];
}
