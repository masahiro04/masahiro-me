export const handleSsr: PagesFunction = async (context) => {
  return await context.next();
};
export const onRequest = [handleSsr];

