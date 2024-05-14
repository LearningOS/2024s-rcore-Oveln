# 实验总结

## 增加内容

1. 增加了spawn系统调用
2. 增加了进程的优先级和stride属性
3. 使用结构体包装线程，然后用BinaryHeap实现优先队列，以实现调度算法

## 问答作业

>stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。
>
>实际情况是轮到 p1 执行吗？为什么？

p2执行，因为+10后，p1.stride=10,p2.stride=5

>我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， 在不考虑溢出的情况下 , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2

优先级大于2，则$pass\in [1,2/BigStride]$

对于任意进程，p1.stride<p2.stride

则进行若干次p1.stride+pass，直到p1.stride>p2.stride

考虑p1.stride>p2.stride的前一次，p1.stride_old+pass>p2.stride

由于$pass\in [1,2/BigStride]$,则p1.stride-p2.stride =pass - (p2.stride-p1.stride_old)

p2.stride-p1.stride_old>=0

则p1.stride-p2.stride<=pass

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.0-other.0).abs() > BigStride / 2 {
            if self.0 > other.0 {
                return (self.0 - BigStride / 2).cmp(other.0);
            } else {
                return self.0.cmp((other.0 - BigStride / 2));
            }
        } else {
            return self.0.cmp(other.0);
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：


1. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

2. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。