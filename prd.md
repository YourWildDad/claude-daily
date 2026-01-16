这是一个可以集成到 ClaudeCode 上的自动记录上下文归档系统。用于自动记录每日 agent 输出的内容，并且给到用户一个系统性反馈。

1. 利用 AgentCli 的 hooks 在每天任务开始的时候在 ~/.claude/daily 文件加下面增加一个 yyyy-mm-dd/ 的文件夹，也提供一个 setup.config ，可以配置用户的存储目录，比如可以配置的到自己的文件夹空间里，可以通过命令行更改。
2. 每次对话结束，hooks 会丢给一个 cli 启动一个后台进程，用于再启一个 claudecode 对当前会话进行归档总结, 这个过程在使用 claude code 的时候不感知，后台进行。追加到 yyyy-mm-dd/[task-name].md 上
3. 在追加完 md 之后，再启动一个异步任务，对今天的你做一个总结，总结到 yyyy-mm-dd/daily.md 上，总是重新思考所有的 [task-name].md 以及总结内容
4. 总结的内容我希望你有一套方法论，用于总结用户每天所思所想，并且提供启发性建议，融会贯通。如果能总结为 skills/command 我希望你能直接提供一个 skills、command 建议，同时提供 /daily-get-skill /daily-get-command 去变成一个可复用的工具
5. 我也希望你提供一个 command 命令或者 cli 命令， /daily-view 可以直接在终端看到当前的 markdown
6. 我希望你完成这个功能，使用上 claude 的 marketplace plugin 下的合适的工具，比如 command、skills、cli

本质核心是让用户无负担的记录每天的工作，能够沉淀为 skills 或者 command 形成上下文复利。

技术栈使用 rust cli，直接可分发
