import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  output: 'export',
  reactStrictMode: true,
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true,
  },
  trailingSlash: true,
  // assetPrefix: '/unbeatable-tictactoe-wasm',
  // basePath: '/unbeatable-tictactoe-wasm'
};

export default nextConfig;
