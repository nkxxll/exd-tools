# todo exd tools

## parser

- write tests for parser
- write impl attributes in the object creators (low prio)

## binding

- [ ] add binding to the elements currently only on the arrow
- [ ] add space to the example 50 not enough
- [ ] calc the connection in the arrow function

## dsl

ideas

1. idea one
```
:start:
node => node1
node 1 => node2
node 2 => node
:end:
```

2. idea two

- start-tree
- start-circle

```
:start-tree:
:c: n -> n1
adlfajasdlfkj
:c: n1 -> n2
asdfjk;lsdkjf
:c: n2 -> n
sd;fkjal;sdkj
:end:
```

3. idea three
```
>>>tree;
n => n1;
<<<;
```

4. idea four
```
:start:
:c: connect node(x, y, s) to node2(x, y, s)
:c: connect node(x, y, s) to node2(x, y, s)
:c: connect node(x, y, s) to node2(x, y, s)
:end:
```

5. idea five tree

```
:start-tree:
root
node1(root) node2(root) node3(root)
node4(node2) node5(node2) node6(node2)
node11(node5)
:end-tree:
```
```
:start-tree:
root;
(root)node1 (root)node2 (root)node3;
(node2)node4 (node2)node5 (node2)node6;
(node5)node11;
:end-tree:
```

- linear:
  - ltr
  - rtl
  - ttb
  - btt

```
:start-linear(ltr):
root;
node1;
node2;
node3;
node4;
:end-linear:
```
