varying vec4 v_color;
varying vec2 v_circle;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 a_circle;
attribute vec4 a_color;

uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;

void main() {
  v_circle = a_circle;
  v_color = a_color;
  gl_Position = u_projection_matrix * u_view_matrix * vec4(a_pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
  if (length(v_circle) > 1.0) {
    discard;
  }
  gl_FragColor = v_color;
}
#endif