<?php

declare(strict_types=1);

final class Csv
{
    public const SEP = ';';

    public function __construct(public string $glue) {}

    /** @param list<string> $xs */
    public function join(array $xs): string
    {
        return implode($this->glue, $xs);
    }
}

/** @return list<string> */
function words(string $s): array
{
    return explode(' ', $s);
}

$parts = explode(',', 'a,b,,c');
$glue = '-';
echo count($parts), ' ', implode($glue, $parts), "\n";
$pair = implode(Csv::SEP, ['x', 'y']);
echo $pair, "\n";
echo (new Csv('+'))->join(words('p q r')), ' ', implode(', ', words('one')), "\n";
