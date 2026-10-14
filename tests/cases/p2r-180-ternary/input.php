<?php

declare(strict_types=1);

enum Size
{
    case Small;
    case Large;
}

final class Person
{
    public function __construct(public string $n, public bool $on) {}

    public function label(): string
    {
        return $this->on ? $this->n : 'none';
    }
}

function label(bool $big): string
{
    return $big ? 'big' : 'small';
}

function pick(bool $c, string $a, string $b): string
{
    return $c ? $a : $b;
}

function half(bool $c): float
{
    return $c ? 1 : 0.5;
}

function maybe(bool $c): ?int
{
    return $c ? 5 : null;
}

function size(int $n): Size
{
    return $n > 10 ? Size::Large : Size::Small;
}

function length(string $s): int
{
    return strlen($s);
}

function run(int $n, string $s): void
{
    $on = $n > 2;
    $sign = $n >= 0 ? 1 : -1;
    $word = $n % 2 === 0 ? 'even' : 'odd';
    $name = 'Ann';
    $greeting = $n > 5 ? $name : 'nobody';
    $total = ($n > 3 ? 10 : 20) + 1;
    $flag = !($n > 3 ? true : false);
    echo $sign, ' ', $word, ' ', $greeting, ' ', $total, ' ', $flag ? 'y' : 'n', "\n";
    $w50 = $on ? 'abcdefghijklm' : 'nopqrstuvwxyz';
    echo $w50, "\n";
    if ($on ? $n > 5 : false) {
        echo "cond\n";
    }
    while ($n > 100 ? true : false) {
        $n = 0;
    }
    $same = $s === ($on ? 'abc' : 'x');
    echo $same ? 'same' : 'diff', "\n";
    $s .= $on ? '!' : '?';
    echo $s, ' x' . ($on ? 'y' : 'z'), "\n";
    $less = ($on ? 1 : 2) < 3;
    echo $less ? 'less' : 'more', "\n";
    $list = [$on ? 1 : 2, 5];
    echo count($list), ' ', length($n > 1 ? 'four' : 'xy'), "\n";
    $m = match ($on ? 1 : 2) {
        1 => 'one',
        default => 'other',
    };
    $up = strtoupper($n > 1 ? $name : 'q');
    echo $m, ' ', $up, "\n";
    $name = $n > 100 ? 'big' : $name;
    echo $name, "\n";
}

echo label(true), ' ', pick(false, 'a', 'b'), ' ', maybe(false) ?? -1, "\n";
echo half(true) > 0.9 ? 'one' : 'half', ' ', size(11) === Size::Large ? 'L' : 'S', "\n";
echo (new Person('Bo', true))->label(), ' ', (new Person('Cy', false))->label(), "\n";
run(7, 'abc');
run(-2, 'xyz');
