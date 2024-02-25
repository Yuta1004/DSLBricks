/** @type {import('next').NextConfig} */

const nextConfig = {
    output: "export",
    distDir: "dist",
    reactStrictMode: true,
    swcMinify: true,
    images: {
        unoptimized: true,
    },
    transpilePackages: ["react-blockly"],
};

module.exports = nextConfig;
