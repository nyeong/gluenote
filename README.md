# GlueSQL Test

[GlueSQL 컨트리뷰션 아카데미](https://www.oss.kr/contribution_22_projects/show/8afa1ae0-c028-4caa-b2fe-9d0fac49cd8c)
참가를 위한 테스트 저장소입니다.

## How to Use

문자열을 저장할 수 있는 노트를 만들 수 있습니다.
`new`, `all`, `update`, `delete` 명령어로 CRUD를 할 수 있습니다.

```bash
$ gluenote
There are no notes here.

$ gluenote new
Type contents^D

$ gluenote all
1. Type content

$ gluenote read 1

$ gluenote update 1
Replace contents^D

$ gluenote delete 1
```