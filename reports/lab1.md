# 实验总结

## 增加内容

1. 在 taskContext 中添加 syscall_info 和 start_time 存储需要新增的系统调用所需要的信息
2. 在 TaskManager 中添加相应的初始化内容
3. 在 TaskManager 中添加获取当前任务状态、当前任务开始时间，以及统计 systemcall 的相关函数，及其包装
4. 添加 sys_task_info 系统调用的具体实现

## 问答作业

### 1

> 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 Rust 两个 bad 测例 (ch2b_bad_*.rs) ) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

1. 对于 bad_address , 内核因为 PageFault panic。然后该 task 退出。
2. 对于 bad_instruction 和 bad_register ， 内核因为 IllegalInstruction panic。然后该 task 退出。

以下是运行环境和程序运行结果:
>[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
>
>[kernel] IllegalInstruction in application, kernel killed it.
>
>[kernel] IllegalInstruction in application, kernel killed it.

RustSBI Version 0.2.0-alpha.2

### 2
> 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:
> 
> L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。
> 
> L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
> 
> L50-L56：为何跳过了 x2 和 x4？
>
> L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
>
> __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
>
> L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？
>
> 从 U 态进入 S 态是哪一条指令发生的？

1. a0是trap_handler的返回值，即TrapContext的地址。
   1. 第一次运行task时从S态进入到U态
   2. 运行完trap_hander后从S态恢复到U态
2. 
   1. t0是sstatus，表示该U态下该task的异常处理的各种状态
   2. t1是sepc，表示U态下该task异常处理函数的位置
   3. t2是sscratch，表示该task的栈帧地址，将在接下来的代码中恢复，并将kernel sp存入到sscratch中
3. x2是sp，会在后面单独处理。x4是线程指针，一般用不到
4. sp是task 用户栈帧，sscratch是该task的kernel栈帧
5. sret，该指令用于从S态下的陷入中恢复。通过sstatus中的SPIE段得知sret应该回到什么态。
6. sp是内核栈帧，sscratch是用户栈帧
7. 从中断或异常触发时，就进入了S态

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

组队队友：金晨烨
内容: riscv指令集相关的内容，以及ch3代码中执行流的内容

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：


1. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

2. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。