<?php

declare(strict_types=1);

/**
 * @param list<string> $xs
 */
function forms(string $s, int $n, array $xs): string
{
    $a = strtr($s, ['a', 'b']);
    $b = strtr($s, [1 => 'one']);
    $c = strtr($s, [$n => 'n']);
    $d = strtr($s, ['a' => $n]);
    $e = strtr($s, ['' => 'x', 'a' => 'b']);
    $f = strtr($s, ['a' => 'b', ...$xs]);
    $g = strtr($s, array('a' => 'b'));
    return $a . $b . $c . $d . $e . $f . $g;
}
