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

const renderHtml = (title: string, description: string, keywords: string, imagePath: string): string => {
  return `
      <!DOCTYPE html>
      <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta name="robots" content="index, follow">
        <meta name="apple-mobile-web-app-status-bar-style" content="black_translucent" />
        <meta name="apple-mobile-web-app-capable" content="yes" />

        <title>${title}</title>
        <meta name="keywords" content="${keywords}" />
        <meta name="description" content="${description}" />

        <meta name="og:title" content="${title}" />
        <meta property="og:image" content="${imagePath}" />
        <meta name="og:description" content={{ post.excerpt() }}/>
        <meta name="twitter:card" content='summary_large_image' />

        <link
          href="https://fonts.googleapis.com/css2?family=Mulish:wght@200&display=swap"
          rel="stylesheet"
        />
        <link href="https://assets.masahiro.me/style.css" rel="stylesheet" />
      </head>
      <body></body>
      </html>`;
};

const handleBot: PagesFunction = async (context) => {
  const originalUrl = context.request.url;

  const splitedUrl = originalUrl.split('/');
  console.log({ splitedUrl });

  const isSocialMediaBot = (userAgent: string): boolean => {
    return ['Twitterbot', 'Slackbot'].some((botUserAgent) => userAgent.includes(botUserAgent));
  };

  const userAgent = context.request.headers.get('user-agent') ?? '';
  if (!isSocialMediaBot(userAgent)) {
    return await context.next();
  }

  if (splitedUrl[3] === 'about') {
    const title = "About me | Masahiro's tech note";
    const imagePath = '/images/kyuri.png';
    const description =
      'ソフトウェアエンジニア、大久保将広のウェブサイトです。現在取り扱っている言語や興味関心ごとなどを記載しております。';
    const keywords = '大久保将広, ソフトウェアエンジニア, バックエンド, フロントエンド, TypeScript, Rust';
    const body = renderHtml(title, description, keywords, imagePath);
    return new Response(body, { headers: { 'Content-Type': 'text/html' } });
  }

  if (splitedUrl[3] === 'projects') {
    const title = "Projects | Masahiro's tech note";
    const imagePath = '/images/kyuri.png';
    const description = '現在参加中の案件一覧です。上流から下流まで対応するプロジェクトやアドバイスを行う顧問活動も行っております。';
    const keywords = '参加案件, ソフトウェアエンジニア, バックエンド, フロントエンド, TypeScript, Rust';
    const body = renderHtml(title, description, keywords, imagePath);
    return new Response(body, { headers: { 'Content-Type': 'text/html' } });
  }

  if (splitedUrl[3] === 'pages' || splitedUrl[3] === '') {
    const title = "Projects | Masahiro's tech note";
    const imagePath = '/images/kyuri.png';
    const description = '現在参加中の案件一覧です。上流から下流まで対応するプロジェクトやアドバイスを行う顧問活動も行っております。';
    const keywords = '参加案件, ソフトウェアエンジニア, バックエンド, フロントエンド, TypeScript, Rust';
    const body = renderHtml(title, description, keywords, imagePath);
    return new Response(body, { headers: { 'Content-Type': 'text/html' } });
  }


  if (!originalUrl.includes('/posts/')) {
    return await context.next();
  }
  return await context.next();

  // const url = new URL(context.request.url);
  // const splitedUrl = url.pathname.split('/');
  // const slug = splitedUrl[splitedUrl.length - 1];
  // const newUrl = new URL(`https://api.masahiro.me/meta?slug=${slug}`);
  // const resp = await fetch(new Request(newUrl));
  // return new Response(resp.body, {
  //   headers: { 'Content-Type': 'text/html' }
  // });
};

export const onRequest: PagesFunction[] = [handleReverseProxy, handleBot];
