<?php

declare(strict_types=1);

final readonly class Address
{
    public function __construct(
        public string $streetNameAndNumber,
        public string $postalCodeAndCity,
        public string $countryOfResidence,
    ) {}

    public function label(string $separatorBetweenParts): string
    {
        return $this->streetNameAndNumber . $separatorBetweenParts . $this->postalCodeAndCity . $separatorBetweenParts . $this->countryOfResidence;
    }
}

function combine(int $firstNumberValue, int $secondNumberValue, int $thirdNumberValue, int $fourth): int
{
    return $firstNumberValue * $secondNumberValue + $thirdNumberValue * $fourth - $firstNumberValue % 7 + $secondNumberValue;
}

$address = new Address('Hoofdstraat 1234 achterom bij de bakker', '5461 AB Veghel in Noord-Brabant', 'Nederland');
echo $address->label(', '), "\n";
echo combine(123456, 234567, 345678, 456789), "\n";
echo 'This is a very long string literal that goes well past the hundred column limit of rustfmt for sure yes', "\n";
$isLongCondition = combine(1, 2, 3, 4) > 10 && combine(5, 6, 7, 8) > 20 && combine(9, 10, 11, 12) > 30;
if ($isLongCondition && combine(1, 2, 3, 4) + combine(5, 6, 7, 8) + combine(9, 10, 11, 12) > 100) {
    echo sprintf('%s and %s and %s', $address->streetNameAndNumber, $address->postalCodeAndCity, $address->countryOfResidence), "\n";
}
