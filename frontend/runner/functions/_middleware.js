import wasmModule from "../../ssr/pkg/ssr_bg.wasm";
import init, { render } from "../../ssr/pkg/ssr.js";

export const handleSsr = async (_) => {
  console.error("sentinel1");
  let result = "";





  const html = `
<!DOCTYPE html><html lang="en"><head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="robots" content="index, follow">
    <meta name="apple-mobile-web-app-status-bar-style" content="black_translucent">
    <meta name="apple-mobile-web-app-capable" content="yes">

    <title>Masahiro's tech note</title>
    <meta name="keywords" content="ITエンジニア, ITコンサル, IT顧問, システム開発, Rust, wasm">
    <meta name="description" content="名古屋のソフトウェアエンジニア。SaaSやマッチングサービス、AR/VR等の開発を経て現在は独立して名古屋で開発やITコンサルしています。サービス開発の所感や、ハマった際の解決方法を記載しております。">
    <meta name="twitter:title" content="Masahiro's tech note">
    <meta name="twitter:description" content="ITエンジニア, ITコンサル, IT顧問, システム開発, Rust, wasm">
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:image" content="/images/kyuri.png">

    <link href="https://fonts.googleapis.com/css2?family=Mulish:wght@200&amp;display=swap" rel="stylesheet">
    <link rel="stylesheet" href="/tailwind-6f8331065adbc9d.css">
    
  
<link rel="preload" href="/runner-d36831d21a081f79_bg.wasm" as="fetch" type="application/wasm" crossorigin="">
<link rel="modulepreload" href="/runner-d36831d21a081f79.js"></head>
  <body>
  

<script type="module">import init from '/runner-d36831d21a081f79.js';init('/runner-d36831d21a081f79_bg.wasm');</script></body></html>
      `;

  console.log("sentinel1");
  await init(wasmModule);
  result = await render(html);
  console.log("sentinel10");
  console.log("sentinel2");
  return new Response(html, {
    headers: {
      "content-type": "text/html;charset=UTF-8",
    },
  });
};

//
export const onRequest = [handleSsr];
//
//

// const handleReverseProxy: PagesFunction = async (context) => {
//   const originalUrl = context.request.url;
//   const url = new URL(originalUrl);
//   if (url.pathname.indexOf('/sitemap') !== 0) {
//     return await context.next();
//   }
//   const newUrl = new URL('https://api.masahiro.me/sitemap');
//   const response = await fetch(new Request(newUrl));
//   return new Response(response.body, {
//     status: response.status,
//     statusText: response.statusText,
//     headers: new Headers(response.headers)
//   });
// };
//
// const handleBot: PagesFunction = async (context) => {
//   const originalUrl = context.request.url;
//   if (!originalUrl.includes('/posts/')) {
//     return await context.next();
//   }
//   const isSocialMediaBot = (userAgent: string): boolean => {
//     return ['Twitterbot', 'Slackbot'].some((botUserAgent) => userAgent.includes(botUserAgent));
//   };
//
//   const userAgent = context.request.headers.get('user-agent') ?? '';
//   if (!isSocialMediaBot(userAgent)) {
//     return await context.next();
//   }
//   const url = new URL(context.request.url);
//   const splitedUrl = url.pathname.split('/');
//   const slug = splitedUrl[splitedUrl.length - 1];
//   const newUrl = new URL(`https://api.masahiro.me/meta?slug=${slug}`);
//   const resp = await fetch(new Request(newUrl));
//   return new Response(resp.body, {
//     headers: { 'Content-Type': 'text/html' }
//   });
// };
