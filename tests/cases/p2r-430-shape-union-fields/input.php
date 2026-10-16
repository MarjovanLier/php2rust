<?php

declare(strict_types=1);

namespace App;

// A declared shape gives its union field the union enum, and a foreach value of the list
// keeps that struct, so the field reads narrow by is_string, is_bool and is_int.
/**
 * @param list<array{key: string, value: string|bool|int}> $data
 */
function describe(array $data): string
{
    $out = '';
    foreach ($data as $item) {
        $value = $item['value'];
        if (is_string($value)) {
            $out .= $item['key'] . '=' . $value . ';';
        } elseif (is_bool($value)) {
            $out .= $item['key'] . '=' . ($value ? 'yes' : 'no') . ';';
        } elseif (is_int($value)) {
            $out .= $item['key'] . '=' . ($value + 1) . ';';
        }
    }
    return $out;
}

/**
 * @param array<array-key, array{name: string, limit: int|string|null}> $rules
 */
function limits(array $rules): string
{
    $out = '';
    foreach ($rules as $rule) {
        $limit = $rule['limit'];
        if ($limit === null) {
            $out .= $rule['name'] . ':none ';
        } elseif (is_int($limit)) {
            $out .= $rule['name'] . ':' . $limit . ' ';
        } else {
            $out .= $rule['name'] . ':text ';
        }
    }
    return $out;
}

/**
 * @return array{label: string, value: float|string}
 */
function measured(bool $exact): array
{
    return ['label' => 'm', 'value' => $exact ? 1.5 : 'about'];
}

echo describe([
    ['key' => 'a', 'value' => 'x'],
    ['key' => 'b', 'value' => true],
    ['key' => 'c', 'value' => 3],
]), "\n";
echo limits(['first' => ['name' => 'a', 'limit' => 5], 'b' => ['name' => 'b', 'limit' => null]]), "\n";
$m = measured(false);
echo $m['label'], ' ', is_string($m['value']) ? 'text' : 'number', "\n";
