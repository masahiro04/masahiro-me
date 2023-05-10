const handleReverseProxy: PagesFunction = async (context) => {
  const originalUrl = context.request.url;
  const url = new URL(originalUrl);
  if (url.pathname.indexOf("/sitemap") !== 0) {
    return await context.next();
  }
  const newUrl = new URL("https://api.masahiro.me/sitemap");
  const response = await fetch(new Request(newUrl));
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: new Headers(response.headers),
  });
};

const handleBot: PagesFunction = async (context) => {
  const originalUrl = context.request.url;
  if (!originalUrl.includes("/posts/")) {
    return await context.next();
  }
  const isSocialMediaBot = (userAgent: string): boolean => {
    return ["Twitterbot", "Slackbot"].some((botUserAgent) =>
      userAgent.includes(botUserAgent)
    );
  };

  const userAgent = context.request.headers.get("user-agent") ?? "";
  if (!isSocialMediaBot(userAgent)) {
    return await context.next();
  }
  const url = new URL(context.request.url);
  const splitedUrl = url.pathname.split("/");
  const slug = splitedUrl[splitedUrl.length - 1];
  const newUrl = new URL(`https://api.masahiro.me/meta?slug=${slug}`);
  const resp = await fetch(new Request(newUrl));
  return new Response(resp.body, {
    headers: { "Content-Type": "text/html" },
  });
};

export const onRequest: PagesFunction[] = [handleReverseProxy, handleBot];
