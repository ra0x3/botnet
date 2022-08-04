export const generateFakeItems = (item: any, count: number): Array<any> => {
  let items = [];

  for (let i = 0; i < count; i++) {
    items.push(item);
  }

  return items;
};
