<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// PHP warns about an octal escape above \377 and keeps one byte of it. P2R-49 does not
// accept such a string, so the model does not read it as its table name and stops at it
// as at any $table that is not a plain string.
final class Flag extends Model
{
    protected $table = "flags\400";
}
