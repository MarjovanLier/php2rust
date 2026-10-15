<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Attributes\MapInputName;
use Spatie\LaravelData\Attributes\MapOutputName;
use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Data as Base;

final class Account extends Data
{
    public function __construct(
        public string $number,
        public ?string $name,
        public bool $blocked,
    ) {}
}

#[MapOutputName('snake')]
class Contact extends Base
{
    public function __construct(
        #[MapInputName('data.email'), Max(255)]
        public string $email,
        #[MapInputName('data.visits')]
        #[Required]
        public int $visits = 0,
    ) {}

    public function visit(): void
    {
        $this->visits++;
    }
}

function label(Account $account): string
{
    $name = $account->name;
    return $account->number . ' ' . ($name ?? 'none') . ($account->blocked ? ' blocked' : '');
}

$account = new Account('1000', null, true);
$contact = new Contact('a@b.c');
$contact->visit();
$contact->visit();
echo label($account), ' ', $contact->email, ' ', $contact->visits, "\n";
