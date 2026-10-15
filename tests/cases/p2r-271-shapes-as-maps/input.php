<?php

declare(strict_types=1);

final readonly class Birth
{
    public function __construct(private string $year, private string $month, private string $day) {}

    /**
     * @return array{year: string, month: string, day: string}
     */
    public function toArray(): array
    {
        return ['year' => $this->year, 'month' => $this->month, 'day' => $this->day];
    }
}

final readonly class Person
{
    public function __construct(private bool $valid, private ?Birth $birth, private ?string $name) {}

    /**
     * @return array<string, bool|array<string, string>|string|null>
     */
    public function toArray(): array
    {
        return [
            'valid' => $this->valid,
            'birth' => $this->birth?->toArray(),
            'name' => $this->name,
        ];
    }
}

final readonly class Card
{
    /**
     * @param array{year: string, month: string, day: string} $parts
     */
    public function __construct(private array $parts) {}

    /**
     * @return array<string, string>
     */
    public function asMap(): array
    {
        return $this->parts;
    }
}

/**
 * @return array{label: string, parts: array<string, string>}
 */
function labelled(Birth $birth): array
{
    return ['label' => 'birth', 'parts' => $birth->toArray()];
}

/**
 * @return array<string, array<string, string>>
 */
function nested(Birth $birth): array
{
    return ['birth' => $birth->toArray()];
}

/**
 * @param array{year: string, month: string, day: string}|null $parts
 *
 * @return array<string, string>|null
 */
function maybe(?array $parts): ?array
{
    return $parts;
}

/**
 * @param array<string, string>|null $map
 */
function described(?array $map): string
{
    if ($map === null) {
        return 'none';
    }
    return implode(',', values($map));
}

/**
 * @return array{dateOfBirth: string, 'home town': string}
 */
function place(string $date, string $town): array
{
    return ['dateOfBirth' => $date, 'home town' => $town];
}

/**
 * @return array<string, string>
 */
function asMap(string $date): array
{
    $where = place($date, 'Durban');
    $map = $where;
    echo $where['home town'], ' ';
    return $map;
}

/**
 * @param array{dateOfBirth: string, 'home town': string} $spot
 */
function joined(array $spot): string
{
    return implode(',', values($spot));
}

/**
 * @param array<string, string> $map
 *
 * @return list<string>
 */
function values(array $map): array
{
    $out = [];
    foreach ($map as $value) {
        $out[] = $value;
    }
    return $out;
}

/**
 * @param array<string, bool|array<string, string>|string|null> $row
 */
function show(array $row): string
{
    $text = '';
    foreach ($row as $value) {
        $text .= gettype($value) . ' ';
    }
    return $text;
}

echo show((new Person(true, new Birth('1980', '01', '02'), 'Ann'))->toArray()), "\n";
echo show((new Person(false, null, null))->toArray()), "\n";
echo implode(',', values(asMap('1980-01-02'))), "\n";
echo joined(['dateOfBirth' => '1990-03-04', 'home town' => 'Cape Town']), "\n";
echo implode(',', values((new Birth('2001', '12', '31'))->toArray())), "\n";
$birth = new Birth('1975', '06', '07');
echo implode(',', values((new Card($birth->toArray()))->asMap())), "\n";
echo labelled($birth)['label'], ' ', implode(',', values(labelled($birth)['parts'])), "\n";
echo count(nested($birth)), ' ', implode(',', values(nested($birth)['birth'])), "\n";
echo described(maybe($birth->toArray())), ' ', described(maybe(null)), "\n";
