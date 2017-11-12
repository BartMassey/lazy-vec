var searchIndex = {};
searchIndex["lazy_vec"] = {"doc":"A \"lazy vector\" is a self-initializing vector: it can be created in constant time, but still has constant-time read and write. It initializes an element on first write, and occupies space proportional to the number of written elements.","items":[[3,"LazyVec","lazy_vec","This opaque structure stores a lazy vector.",null,null],[11,"new","","Allocate a new empty `LazyVec`.",0,{"inputs":[],"output":{"name":"lazyvec"}}],[11,"with_capacity","","Allocate a new empty `LazyVec` with the given starting index capacity.",0,{"inputs":[{"name":"usize"}],"output":{"name":"lazyvec"}}],[11,"cap","","Number of elements there is currently notional capacity for, including uninitialized ones.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len","","Number of elements notionally stored, including uninitialized ones.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"value_ref","","Return a reference to the value at the given index.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"t"}}],[11,"value_ref_mut","","Return a mutable reference to the value at index `i`. If no value previously existed, this will return a reference to uninitialized memory, making it unsafe.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"t"}}],[11,"index","","",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"t"}}],[11,"index_mut","","",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"t"}}]],"paths":[[3,"LazyVec"]]};
initSearch(searchIndex);
