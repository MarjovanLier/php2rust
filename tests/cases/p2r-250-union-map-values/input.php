<?php

declare(strict_types=1);

function check(string $id): ?bool
{
    return strlen($id) === 13 ? true : null;
}

/**
 * The validator's batchValidate: a map of scalar values, keeping the strings.
 *
 * @param array<int|string, scalar> $ids
 * @return array<string, ?bool>
 */
function batch(array $ids): array
{
    $results = [];
    foreach ($ids as $id) {
        if (!\is_string($id)) {
            continue;
        }
        $results[$id] = check($id);
    }
    return $results;
}

/**
 * The declared type the map flows into wins over Mago's narrower types.
 *
 * @return array<string, int|string>
 */
function build(string $k): array
{
    $m = [];
    $m[$k] = 5;
    $m['b'] = 'x';
    return $m;
}

/**
 * @param array<string, int|string> $m
 */
function describe(array $m): string
{
    $out = '';
    foreach ($m as $v) {
        if (is_int($v)) {
            $out .= 'i' . ($v + 1);
        } elseif (is_string($v)) {
            $out .= 's' . strlen($v);
        }
    }
    return $out . count($m) . (isset($m['n']) ? 'y' : 'n');
}

/**
 * @param array<string, int|string|null> $m
 */
function nulls(array $m): int
{
    $n = 0;
    foreach ($m as $v) {
        if ($v !== null && is_string($v)) {
            $n += strlen($v);
        }
    }
    return $n;
}

$r = batch(['8001015009087', 5, 'abc', true, 1.5]);
echo count($r), isset($r['abc']) ? ' set' : ' unset', array_key_exists('abc', $r) ? ' has' : ' not', "\n";
echo describe(build('a')), ' ', describe(['a' => 4, 'n' => 'q', 'c' => 'xyz']), "\n";
echo nulls(['a' => 'abc', 'b' => null, 'c' => 1]), "\n";
