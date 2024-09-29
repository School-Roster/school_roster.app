/**
  * Funcion para calcular el color de texto que se debe usar en base al color de fondo
  * @param {string} hex - Color de fondo en formato hexadecimal
  */
export function getContrastColor(hex: string) {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  const yiq = (r * 299 + g * 587 + b * 114) / 1000; // algoritmo para calcular el brillo (YIQ)
  return yiq >= 128 ? "#000000" : "#ffffff"; // TODO: Esto podria usar el color de texto del tema actual
}
