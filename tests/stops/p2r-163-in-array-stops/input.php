<?php

declare(strict_types=1);

enum Suit
{
    case Hearts;
    case Spades;
}

final class Limits
{
    const MAX = 3;
}

/**
 * @param list<int> $list
 */
function check(int $n, float $f, array $list, ?int $maybe, string $s): void
{
    $a = in_array($n, [1, 2]);
    $b = in_array($n, [1, 2], false);
    $c = in_array($n, $list, true);
    $d = in_array($n, [], true);
    $e = in_array(1, [9223372036854775808], true);
    $g = in_array($n, [1, 'a'], true);
    $h = in_array($maybe, [1], true);
    $i = in_array($f, [1.5], true);
    $j = in_array($n, [Limits::MAX], true);
    $k = in_array($s, ["a$n"], true);
    $l = in_array($n, [1 + 1], true);
    $m = in_array(Suit::Hearts, [Suit::Spades, 1], true);
    $o = in_array($n, [$n], true);
    echo $a, $b, $c, $d, $e, $g, $h, $i, $j, $k, $l, $m, $o;
}
