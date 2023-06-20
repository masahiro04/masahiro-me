// export const handleSsr: PagesFunction = async (_) => {
//   const html = 'aaa';
//   return new Response(html, {
//     status: 200,
//     headers: {
//       'content-type': 'text/plain;charset=UTF-8'
//     }
//   });
// };
//
// export const onRequest: PagesFunction[] = [handleSsr];
//
//

const handleReverseProxy: PagesFunction = async (context) => {
  const originalUrl = context.request.url;
  const url = new URL(originalUrl);

  console.log({ pathname: url.pathname, startWith: url.pathname.startsWith("/pages"), res: url.pathname.indexOf("/posts") });
  const isFetchData = url.pathname.startsWith("/pages") || url.pathname.startsWith("/posts");
  if (isFetchData) {
    console.log("haotta!");

    const newUrl = new URL(
      `https://masahiro-me-3s6gqspbuq-an.a.run.app/${url.pathname}`
    );
    console.log({ newUrl });
    const response = await fetch(new Request(newUrl));
    return new Response(response.body, {
      status: response.status,
      statusText: response.statusText,
      headers: new Headers(response.headers),
    });
    // return await context.next();
  }

  // if (url.pathname.indexOf("/sitemap") !== 0) {
  //   const newUrl = new URL("https://masahiro-me-3s6gqspbuq-an.a.run.app");
  //   console.log({ url });
  //   const response = await fetch(new Request(newUrl));
  //   return new Response(response.body, {
  //     status: response.status,
  //     statusText: response.statusText,
  //     headers: new Headers(response.headers),
  //   });
  //   // return await context.next();
  // }
  const newUrl = new URL("https://api.masahiro.me/sitemap");
  const response = await fetch(new Request(newUrl));
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: new Headers(response.headers),
  });
};

// const handleBot: PagesFunction = async (context) => {
//   const originalUrl = context.request.url;
//   if (!originalUrl.includes("/posts/")) {
//     return await context.next();
//   }
//   const isSocialMediaBot = (userAgent: string): boolean => {
//     return ["Twitterbot", "Slackbot"].some((botUserAgent) =>
//       userAgent.includes(botUserAgent)
//     );
//   };
//
//   const userAgent = context.request.headers.get("user-agent") ?? "";
//   if (!isSocialMediaBot(userAgent)) {
//     return await context.next();
//   }
//   const url = new URL(context.request.url);
//   const splitedUrl = url.pathname.split("/");
//   const slug = splitedUrl[splitedUrl.length - 1];
//   const newUrl = new URL(`https://api.masahiro.me/meta?slug=${slug}`);
//   const resp = await fetch(new Request(newUrl));
//   return new Response(resp.body, {
//     headers: { "Content-Type": "text/html" },
//   });
// };

export const onRequest: PagesFunction[] = [handleReverseProxy];
