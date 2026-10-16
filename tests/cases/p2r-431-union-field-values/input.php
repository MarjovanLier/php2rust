<?php

declare(strict_types=1);

namespace App;

// A union field takes each member value in a literal, and a read of it goes into a place
// of its own union: a parameter, a return value and a property.
final class Setting
{
    public function __construct(public readonly int|string $value)
    {
    }
}

function show(int|string $value): string
{
    return is_int($value) ? 'int' : 'text';
}

/**
 * @param array{id: int, value: int|string} $row
 */
function valueOf(array $row): int|string
{
    return $row['value'];
}

/**
 * @return array{id: int, value: int|string}
 */
function entry(int $id, string $text): array
{
    return ['id' => $id, 'value' => $id > 1 ? $text : $id];
}

$first = entry(1, 'one');
$second = entry(2, 'two');
echo show($first['value']), ' ', show($second['value']), "\n";
$kept = valueOf($second);
echo show($kept), ' ', $first['id'], "\n";
$setting = new Setting($second['value']);
echo show($setting->value), "\n";
