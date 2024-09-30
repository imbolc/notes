// Loops

// for...of - variables are scoped
for (const i of [1, 2, 3]) {
  let x = i * 2;
  setTimeout(() => {
    console.log({ i, x });
  }, 100);
}
