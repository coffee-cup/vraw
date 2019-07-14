pub fn get_stdlib() -> String {
    r#"
shape circle(
  cx=0,
  cy=0,
  r=10,
  fill="hotpink",
  stroke="none",
  strokeWidth=0) {

  svg(value: "<circle
    cx=\"" + cx + "\"
    cy=\"" + cy + "\"
    r=\"" + r + "\"
    fill=\"" + fill + "\"
    stroke=\"" + stroke + "\"
    strokeWidth=\"" + strokeWidth + "\"
  />")
}

shape rect(
  x=0,
  y=0,
  width=10,
  height=10,
  fill="hotpink",
  stroke="none",
  strokeWidth=0) {

  svg(value: "<rect
    x=\"" + x + "\"
    y=\"" + y + "\"
    width=\"" + width + "\"
    height=\"" + height + "\"
    fill=\"" + fill + "\"
    stroke=\"" + stroke + "\"
    strokeWidth=\"" + strokeWidth + "\"
  />")
}
"#
    .to_owned()
}
