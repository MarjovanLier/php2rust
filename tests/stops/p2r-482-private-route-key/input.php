<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// A private getRouteKeyName of Model does not count for #[\Override], which PHP refuses
// (P2R-480, P2R-482).
final class Account extends Model
{
    public $timestamps = false;

    #[\Override]
    public function getRouteKeyName(): string
    {
        return 'uuid';
    }
}
