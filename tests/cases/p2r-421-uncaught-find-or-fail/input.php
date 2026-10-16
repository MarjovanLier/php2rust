<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

Code::create(['name' => 'a']);
$found = Code::findOrFail(1);
echo $found->name, "\n";
// No row has id 5, so the run stops here with exit code 255, as PHP's does.
$missing = Code::findOrFail(5);
echo $missing->name, "\n";
