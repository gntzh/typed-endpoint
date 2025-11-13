# typed-endpoint

编译器组织endpoint的实践，主要使用 Type-level Linked List 实现对树形路径结构的递归构建。


没什么实际用处，一、树形Endpoint并不需要，实践中只需一个相对base url的完整path或者path模板即可；二、单纯的路径组织没必要，需要和Path Params、Request、Response配合。
