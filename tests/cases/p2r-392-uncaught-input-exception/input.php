<?php

declare(strict_types=1);

namespace App\Exceptions;

final class AdministrationCompanyNumberNotSetException extends \Exception
{
    public function __construct(string $companyNumberFromFile)
    {
        parent::__construct(
            "No administration found for company number: {$companyNumberFromFile}. " .
            'Please ensure all administrations have company_number set.'
        );
    }
}

function find(string $number): string
{
    if ($number === '') {
        throw new AdministrationCompanyNumberNotSetException($number);
    }
    return 'found ' . $number;
}

echo find('42'), "\n";
// PHP stops here with an uncaught exception of the input's class (P2R-216).
echo find(''), "\n";
echo "not reached\n";
