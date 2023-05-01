const handleReverseProxy: PagesFunction = async (context) => {
  const originalUrl = context.request.url;
  const url = new URL(originalUrl);
  if (url.pathname.indexOf('/sitemap') !== 0) {
    return await context.next();
  }
  const newUrl = new URL('https://api.masahiro.me/sitemap');
  const response = await fetch(new Request(newUrl));
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: new Headers(response.headers)
  });
};

export const onRequest: PagesFunction[] = [
  handleReverseProxy,
];

