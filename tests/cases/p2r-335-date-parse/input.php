<?php

declare(strict_types=1);

final class Dates
{
    public static function isValidDate(string $date, string $format = 'Y-m-d H:i:s'): bool
    {
        $dateTime = DateTime::createFromFormat($format, $date);

        if (!$dateTime instanceof DateTime || $dateTime->format($format) !== $date) {
            return false;
        }

        /**
         * @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $dateParts
         */
        $dateParts = \date_parse($dateTime->format('Y-m-d H:i:s'));

        return self::isValidTimePart($dateParts);
    }

    /**
     * @param array{year: int, month: int, day: int, hour: int, minute: int, second: int} $dateParts
     */
    private static function isValidTimePart(array $dateParts): bool
    {
        if (!\checkdate($dateParts['month'], $dateParts['day'], $dateParts['year'])) {
            return false;
        }
        return $dateParts['hour'] < 24 && $dateParts['minute'] < 60 && $dateParts['second'] < 60;
    }

    public static function parts(string $text): string
    {
        $date = DateTime::createFromFormat('!Y-m-d H:i:s', $text);
        if ($date === false) {
            return 'false';
        }
        /** @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $parts */
        $parts = date_parse($date->format('Y-m-d H:i:s'));
        return $parts['year'] . '/' . $parts['month'] . '/' . $parts['day'] . ' ' . $parts['hour'] . ':'
            . $parts['minute'] . ':' . $parts['second'];
    }

    public static function twice(string $a, string $b): int
    {
        $first = DateTime::createFromFormat('!Y-m-d', $a);
        $second = DateTime::createFromFormat('!Y-m-d', $b);
        if ($first === false) {
            return 0;
        }
        if ($second === false) {
            return 0;
        }
        /** @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $parts */
        $parts = date_parse($first->format('Y-m-d H:i:s'));
        $year = $parts['year'];
        /** @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $parts */
        $parts = date_parse($second->format('Y-m-d H:i:s'));
        return $year + $parts['year'];
    }
}

foreach (['19800229', '19810229', '19800230', '1980021', '00000101', '99991231', 'x'] as $date) {
    echo Dates::isValidDate($date, 'Ymd') ? 'y' : 'n';
}
echo "\n";
foreach (['2023-09-06 12:30:00', '2012-02-30 12:12:12', '2023-12-25 25:00:00', '2023-09-06'] as $date) {
    echo Dates::isValidDate($date) ? 'y' : 'n';
}
echo "\n";
echo Dates::parts('1980-02-29 23:59:59'), ' ', Dates::parts('0000-00-00 00:00:00'), "\n";
echo Dates::twice('1980-01-01', '2000-12-31'), ' ', Dates::twice('x', '2000-12-31'), "\n";
