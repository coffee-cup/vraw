import("../../crate/pkg/index.js")
  .then(module => {
    console.log(module);

    const foo = module.bar("test");
    console.log(foo);

    console.log(foo.go_riders("test"));
  })
  .catch(console.error);
