<?php

declare(strict_types=1);

final readonly class Boss
{
    public function __construct(
        public ?string $nick,
        public int|string $id,
    ) {}
}

final readonly class Person
{
    public function __construct(
        public ?string $nick,
        public int|string $id,
        public ?Boss $boss,
    ) {}

    public function code(): int|string|null
    {
        return $this->nick;
    }

    public function key(): int|string
    {
        return $this->id;
    }
}

function fromParam(?string $s): int|string|null
{
    return $s;
}

function same(int|string $v): int|string
{
    return $v;
}

function maybe(?int $n): int|string|null
{
    $m = $n;
    return $m;
}

function show(int|string|null $v): string
{
    return gettype($v);
}

function pick(bool $c, string $s, int $n): string
{
    return show($c ? $s : $n);
}

function chosen(bool $c, string $s): int|string
{
    return match ($c) {
        true => $s,
        false => 0,
    };
}

final class Config
{
    public int|string $mode = 'auto';
}

function either(int|string|null $a, int|string $b): int|string
{
    return $a ?? $b;
}

function bound(int|string|null $v): string
{
    if ($v !== null) {
        return gettype($v);
    }
    return 'none';
}

function copied(int|string $v): int|string
{
    $w = $v;
    $x = $w;
    echo gettype($w), ' ';
    return $x;
}

$p = new Person('bo', 7, null);
$q = new Person(null, 'x9', new Boss('big', 1));
echo show($p->code()), ' ', show($q->code()), ' ', show($p->key()), ' ', show($q->key()), "\n";
echo show(fromParam('a')), ' ', show(fromParam(null)), ' ', show(same(3)), "\n";
echo show(maybe(5)), ' ', show(maybe(null)), "\n";
echo pick(true, 'a', 1), ' ', pick(false, 'a', 1), "\n";
echo show(chosen(true, 'z')), ' ', show(chosen(false, 'z')), "\n";
echo show(copied('c')), "\n";
$config = new Config();
echo show($config->mode), ' ', show(either(null, 4)), ' ', show(either('s', 4)), "\n";
echo bound(3), ' ', bound(null), "\n";
