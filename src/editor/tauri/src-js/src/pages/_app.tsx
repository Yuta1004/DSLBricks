import type { AppProps } from "next/app";

export default function EditorApp({ Component, pageProps }: AppProps) {
    return <Component {...pageProps} />;
}
