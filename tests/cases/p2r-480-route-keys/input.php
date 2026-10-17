<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Override;

// getRouteKeyName of P2R-480 leaves the output, as casts() does: with #[\Override], with
// #[Override] after `use Override;`, with no attribute, and returning a double-quoted
// string. Laravel calls it only from route binding, which nothing here reaches.
final class Account extends Model
{
    protected $fillable = ['uuid', 'name'];

    public $timestamps = false;

    #[\Override]
    public function getRouteKeyName(): string
    {
        return 'uuid';
    }
}

final class Code extends Model
{
    protected $fillable = ['code'];

    public $timestamps = false;

    #[Override]
    public function getRouteKeyName(): string
    {
        return "code";
    }
}

final class Source extends Model
{
    protected $table = 'sources';

    protected $fillable = ['label'];

    public $timestamps = false;

    public function GETROUTEKEYNAME(): string
    {
        return 'label';
    }
}

function nameOf(int $id): string
{
    $account = Account::find($id);
    return $account === null ? '-' : $account->name;
}

$account = Account::create(['uuid' => 'a-1', 'name' => 'Ann']);
$code = Code::create(['code' => 'X1']);
$source = Source::create(['label' => 'bank']);
echo $account->uuid, ' ', nameOf(1), ' ', nameOf(2), ' ', $code->code, ' ', $source->label, "\n";
