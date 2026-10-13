<?php

declare(strict_types=1);

final class Tank
{
    public int $level = 0;

    public function fill(int $n): void
    {
        $this->level += $n;
    }

    public function read(): int
    {
        return $this->level;
    }

    public function ping(int $n): int
    {
        if ($n <= 0) {
            return $this->level;
        }
        return $this->pong($n - 1);
    }

    public function pong(int $n): int
    {
        return $this->ping($n - 1);
    }

    public function top(): void
    {
        $this->fill(1);
    }
}

final class Fleet
{
    /**
     * @param list<Tank> $tanks
     */
    public function __construct(public array $tanks)
    {
    }
}

/**
 * @param list<Tank> $tanks
 */
function levels(array $tanks): int
{
    $sum = 0;
    foreach ($tanks as $tank) {
        $sum += $tank->read();
    }
    return $sum;
}

/**
 * @param list<Tank> $tanks
 */
function fillAll(array $tanks): void
{
    foreach ($tanks as $tank) {
        $tank->fill(2);
    }
}

/**
 * @param list<Tank> $tanks
 */
function keepAll(array $tanks): Fleet
{
    return new Fleet($tanks);
}

function show(Tank $tank): void
{
    echo $tank->read(), "\n";
}

function change(Tank $tank): void
{
    $tank->fill(3);
}

function adopt(Tank $tank): Fleet
{
    $tank->fill(1);
    return new Fleet([$tank]);
}

$a = new Tank();
$a->top();
change($a);
show($a);
echo $a->ping(3), "\n";
$fleet = adopt($a);
fillAll($fleet->tanks);
echo levels($fleet->tanks), "\n";
$other = keepAll([new Tank(), new Tank()]);
echo levels($other->tanks), "\n";
