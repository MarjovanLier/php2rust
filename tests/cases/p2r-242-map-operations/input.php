<?php

declare(strict_types=1);

/**
 * @param list<string> $ids
 * @return array<string, ?bool>
 */
function check(array $ids): array
{
    $results = [];
    foreach ($ids as $id) {
        $results[$id] = strlen($id) > 2 ? true : null;
    }
    return $results;
}

/**
 * @param array<array-key, int> $m
 */
function look(array $m, string $k, int $n): int
{
    $a = $m['a'];
    $b = $m[$k] ?? 7;
    $c = isset($m[$k]) ? 1 : 0;
    $d = array_key_exists($k, $m) ? 1 : 0;
    $e = $m[$n] ?? 9;
    return $a + $b + $c + $d + $e + $m[16];
}

/**
 * @param array<array-key, int> $m
 * @return array<array-key, int>
 */
function change(array $m): array
{
    $m['z'] = 26;
    $m[5] = 5;
    $m['a'] = 10;
    unset($m['b'], $m['missing']);
    return $m;
}

/**
 * @param array<int, string> $m
 */
function ints(array $m, int $i): string
{
    $m[$i] = 'new';
    $m[-1] = 'neg';
    unset($m[3]);
    $out = '';
    foreach ($m as $k => $v) {
        $out .= $k . ':' . $v . ',';
    }
    return $out . ($m[5] ?? 'none') . (isset($m[-1]) ? 'y' : 'n');
}

/**
 * @param array<string, string> $m
 */
function texts(array $m): string
{
    $s = $m['k'];
    $t = $m['q'] ?? 'none';
    $all = '';
    foreach ($m as $v) {
        $all .= $v;
    }
    return $s . $t . $all . $m['k'];
}

/**
 * @param array<string, int> $m
 */
function total(array $m): int
{
    $sum = 0;
    foreach ($m as $v) {
        $sum += $v;
    }
    return $sum + count($m);
}

$r = check(['abc', 'x', 'abcd']);
echo count($r), ' ', isset($r['x']) ? 'set' : 'unset', ' ', array_key_exists('x', $r) ? 'has' : 'not', "\n";
echo look(['a' => 1, 'b' => 2, '16' => 4], 'b', 16), "\n";
echo look(['a' => 1, 16 => 0], 'q', 3), "\n";
$c = change(['a' => 1, 'b' => 2, 'c' => 3]);
foreach ($c as $v) {
    echo $v, ' ';
}
echo count($c), "\n";
echo ints([3 => 'c', 1 => 'a'], 1), "\n";
echo ints([5 => 'e'], 7), "\n";
echo texts(['k' => 'K', 'j' => 'J']), "\n";
echo total(['a' => 1, 'b' => 2]), "\n";
