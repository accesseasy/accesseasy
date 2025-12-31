const express = require('express');
const path = require('path');
const fs = require('fs').promises;
const { existsSync } = require('fs');
const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());
// Serve static files from repository root
app.use(express.static(path.join(__dirname)));

const EVENTS_FILE = path.join(__dirname, 'events.json');

async function readEvents() {
  try {
    if (!existsSync(EVENTS_FILE)) return [];
    const data = await fs.readFile(EVENTS_FILE, 'utf8');
    return JSON.parse(data || '[]');
  } catch (err) {
    console.error('Error reading events.json', err);
    return [];
  }
}

async function writeEvents(events) {
  try {
    await fs.writeFile(EVENTS_FILE, JSON.stringify(events, null, 2), 'utf8');
  } catch (err) {
    console.error('Error writing events.json', err);
    throw err;
  }
}

app.post('/api/events', async (req, res) => {
  try {
    const event = req.body || {};
    // basic server-side enrichment
    event.id = Date.now().toString();
    event.created_at = new Date().toISOString();

    const events = await readEvents();
    events.push(event);
    await writeEvents(events);

    res.status(201).json({ success: true, event });
  } catch (err) {
    console.error(err);
    res.status(500).json({ success: false, error: 'Failed to save event' });
  }
});

app.get('/api/events', async (req, res) => {
  const events = await readEvents();
  res.json(events);
});

app.listen(PORT, () => {
  console.log(`Server listening on http://localhost:${PORT}`);
});
