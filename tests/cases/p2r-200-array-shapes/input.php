<?php

declare(strict_types=1);

final readonly class DateParts
{
    public function __construct(
        public string $year,
        public string $month,
        public string $day,
    ) {}

    /**
     * @param array{year: string, month: string, day: string} $data
     */
    public static function fromArray(array $data): self
    {
        return new self(
            year: $data['year'],
            month: $data['month'],
            day: $data['day'],
        );
    }

    /**
     * @return array{year: string, month: string, day: string}
     */
    public function toArray(): array
    {
        return [
            'year' => $this->year,
            'month' => $this->month,
            'day' => $this->day,
        ];
    }
}

/**
 * @return array{year: string, month: string, day: string}
 */
function split(string $date): array
{
    $year = substr($date, 0, 4);
    $month = substr($date, 5, 2);
    $day = substr($date, 8, 2);
    return ['year' => $year, 'month' => $month, 'day' => $day];
}

/**
 * @param array{year: string, month: string, day: string} $parts
 */
function glue(array $parts): string
{
    return $parts['day'] . '/' . $parts['month'] . '/' . $parts['year'];
}

/**
 * @return array{year: string, month: string, day: string}|null
 */
function maybe(string $s): ?array
{
    if (strlen($s) !== 10) {
        return null;
    }
    return split($s);
}

/**
 * @return array{name: string, age: int, score: float, tags: list<string>, note: ?string}
 */
function person(string $name, int $age): array
{
    return ['name' => $name, 'age' => $age, 'score' => 1.5, 'tags' => ['a', 'b'], 'note' => null];
}

/**
 * @return list<array{x: int, y: int}>
 */
function points(): array
{
    return [['x' => 1, 'y' => 2], ['y' => 4, 'x' => 3]];
}

/**
 * @return array{when: array{year: string, month: string, day: string}, label: string}
 */
function event(): array
{
    return ['label' => 'launch', 'when' => split('2026-10-14')];
}

function dayOf(?DateParts $parts): string
{
    return $parts === null ? 'none' : $parts->day;
}

function say(string $s): string
{
    echo $s;
    return $s;
}

echo glue(split('2026-10-14')), "\n";
$d = split('2025-01-02');
$copy = $d;
echo glue($d), ' ', glue($copy), "\n";
$m = maybe('2024-12-31');
$text = $m === null ? 'none' : $m['day'];
echo $text, "\n";
if ($m !== null) {
    echo glue($m), "\n";
}
$none = maybe('x');
echo $none === null ? 'no date' : 'a date', "\n";
$p = person('Ann', 41);
echo $p['name'], ' ', $p['age'], ' ', count($p['tags']), "\n";
$total = 0;
foreach (points() as $pt) {
    $total += $pt['x'] * $pt['y'];
}
echo $total, "\n";
$e = event();
echo $e['label'], ' ', glue($e['when']), "\n";
$order = ['z' => say('1'), 'a' => say('2'), 'm' => say('3')];
echo "\n", $order['a'], $order['m'], $order['z'], "\n";
$dp = DateParts::fromArray(split('1999-07-04'));
echo glue($dp->toArray()), "\n";
$x = maybe('2000-02-29');
echo dayOf($x !== null ? DateParts::fromArray($x) : null), "\n";
echo dayOf(null), "\n";
