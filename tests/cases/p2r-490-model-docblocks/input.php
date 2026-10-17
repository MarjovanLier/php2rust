<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// Docblock types narrower than Laravel's on the configuration properties that P2R-401 reads
// as literals and leaves out: Mago's incompatible-property-type finding on them does not
// stop (P2R-490), because PHP never checks a docblock.
final class LocationCode extends Model
{
    /**
     * @var list<string>
     */
    protected $fillable = ['name', 'code', 'secret'];

    /**
     * @var list<string>
     */
    protected $hidden = ['secret'];

    /**
     * @var array<string, string>
     */
    protected $casts = ['code' => 'string'];

    /**
     * @var bool
     */
    public $timestamps = false;
}

final class Event extends Model
{
    /**
     * @var string
     */
    protected $table = 'events';

    /**
     * @var list<string>
     */
    protected $guarded = [];

    /**
     * @var false
     */
    public $timestamps = false;
}

function codeOf(int $id): string
{
    $code = LocationCode::find($id);
    return $code === null ? '-' : $code->code;
}

$code = LocationCode::create(['name' => 'Office', 'code' => 'OF', 'secret' => 's']);
$event = Event::create(['label' => 'opened']);
echo $code->id, ' ', $code->name, ' ', codeOf(1), ' ', codeOf(2), ' ', $event->label, "\n";
