<?php

declare(strict_types=1);

function pair(string $a, string $b): string
{
    return $a . '+' . $b;
}

function fix(?string $name): ?string
{
    if ($name === null) {
        return null;
    }
    $name = trim($name);
    $name = pair($name, $name);
    if (str_contains($name, 'x')) {
        $name = str_replace('x', 'y', $name);
    }
    $name .= '!';
    return $name;
}

function before(?string $name, bool $flag): string
{
    if ($flag) {
        $name = null;
    }
    if (null === $name) {
        return 'none';
    }
    $name = strtoupper($name);
    return $name;
}

function bump(?int $n): int
{
    if ($n === null) {
        return 0;
    }
    $n = $n + 1;
    $n++;
    --$n;
    $n *= 3;
    return $n * 2;
}

/** @param ?list<int> $xs */
function grow(?array $xs): int
{
    if ($xs === null) {
        return -1;
    }
    $xs[] = 7;
    return count($xs);
}

/** @param ?array<string, int> $m */
function put(?array $m): int
{
    if ($m === null) {
        return 0;
    }
    $m['k'] = 5;
    return count($m) * 10 + $m['k'];
}

/** @param ?array<string, int> $m */
function drop(?array $m): int
{
    if ($m === null) {
        return -1;
    }
    unset($m['a']);
    return count($m);
}

enum Suit: string
{
    case Hearts = 'h';
    case Spades = 's';
}

function scalars(?float $f, ?bool $b, ?Suit $s): string
{
    if ($f === null) {
        return 'no float';
    }
    if ($b === null) {
        return 'no bool';
    }
    if ($s === null) {
        return 'no suit';
    }
    $f *= 2.5;
    $b = !$b;
    $s = $s === Suit::Hearts ? Suit::Spades : Suit::Hearts;
    return ($f > 4.0 ? 'big' : 'small') . ' ' . ($b ? 'yes' : 'no') . ' ' . $s->value;
}

function maybe(string $s): ?string
{
    if ($s === '') {
        return null;
    }
    return $s;
}

function local(string $s): string
{
    $v = maybe($s);
    if ($v === null) {
        return 'empty';
    }
    $v = $v . $v;
    return $v;
}

final class Names
{
    public static function shout(?string $name): string
    {
        if ($name === null) {
            throw new InvalidArgumentException('no name');
        }
        $name = strtoupper($name);
        if (strlen($name) > 3) {
            $name = substr($name, 0, 3);
        }
        return $name;
    }
}

echo fix(' axe ') ?? 'null', "\n";
echo fix(null) ?? 'null', "\n";
echo before('ab', false), ' ', before('ab', true), "\n";
echo bump(4), ' ', bump(null), "\n";
echo grow([1, 2]), ' ', grow(null), "\n";
echo put(['a' => 1]), ' ', put(null), ' ', put(['k' => 2]), "\n";
echo drop(['a' => 1, 'b' => 2]), ' ', drop(null), "\n";
echo scalars(1.5, false, Suit::Hearts), ' ', scalars(null, true, null), ' ',
    scalars(2.0, null, null), ' ', scalars(2.0, true, null), "\n";
echo local('ab'), ' ', local(''), "\n";
echo Names::shout('anna'), ' ', Names::shout('al'), "\n";
