export async function initWasm() {
  const wasm = await import('@/../public/wasm/pkg/unbeatable_tictactoe.js');
  await wasm.default(); // Initialize
  return wasm;
}
