<?php

declare(strict_types=1);

final readonly class Point
{
    public function __construct(
        public int $x,
        public int $y,
    ) {}

    public function sum(): int
    {
        return $this->x + $this->y;
    }

    public function display(): void
    {
        show($this);
    }
}

final readonly class Holder
{
    public function __construct(
        public string $name,
        public ?Point $point,
    ) {}

    public function describe(): string
    {
        $p = $this->point;
        if ($p !== null) {
            return $this->name . ': ' . $p->sum();
        }
        return $this->name . ': none';
    }

    public function pointOrNull(): ?Point
    {
        return $this->point;
    }
}

function find(int $n): ?Point
{
    if ($n > 0) {
        return new Point($n, $n * 2);
    }
    return null;
}

function total(Point $p): int
{
    return $p->sum();
}

function show(?Point $p): void
{
    if ($p === null) {
        echo "nothing\n";
        return;
    }
    echo total($p), "\n";
}

function keep(?Point $p): ?Point
{
    return $p;
}

function wrap(Point $p): ?Point
{
    show($p);
    return $p;
}

$a = find(2);
$b = find(0);
show($a);
show($b);
show(new Point(1, 1));
show(null);
$h = new Holder('h', $a);
echo $h->describe(), "\n";
$g = new Holder('g', null);
echo $g->describe(), "\n";
show($h->point);
show($g->pointOrNull());
show(keep($b));
show(find(5));
$q = new Point(3, 4);
$q->display();
show(wrap($q));
$m = $h->pointOrNull();
$r = $m ?? $q;
echo $r->sum(), "\n";
echo $a === null ? "a null\n" : "a set\n";
