import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { LineChart, Line, XAxis, YAxis, CartesianGrid } from "recharts";
import { useMemo } from "react";

type Project = {
  id: string;
  color: string;
  title: string;
  reason: string;
}

type Contribution = {
  project_id: string,
  date: string,
  number: number,
}

function App() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [contributions, setContributions] = useState<Contribution[]>([]);
  const [loading, setLoading] = useState(true);
  const [color, setColor] = useState("#000000");
  const [title, setTitle] = useState("");
  const [reason, setReason] = useState("");
  const [onCreateProject, setOnCreateProject] = useState(false);
  const [hoveredProjectId, setHoveredProjectId] = useState<string | null>(null);

  function resetFields() {
    setColor("#000000");
    setTitle("");
    setReason("");
  }
  const chartData = useMemo(() => {
    const data: Record<string, any>[] = [];
    contributions.forEach(c => {
      let entry = data.find(d => d.date === c.date);
      if (!entry) {
        entry = { date: c.date };
        data.push(entry);
      }
      entry[c.project_id] = c.number;
    });
    data.sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime());
    return data;
  }, [contributions]);

  const projectKeys = Array.from(new Set(contributions.map(c => c.project_id)));

  async function get_projects() {
    setProjects(await invoke("get_projects", {}));
  }

  useEffect(() => {
    async function load() {
      const data: Project[] = await invoke("get_projects");
      setProjects(data);
      setLoading(false);
    }
    load();
  }, []);

  useEffect(() => {
    async function loadContributions() {
      const data: Contribution[] = await invoke("get_contributions");
      setContributions(data);
    }
    loadContributions();
    const unlisten = listen("contributions_updated", () => {
      loadContributions();
    });
    return () => {
      unlisten.then(f => f());
    };
  }, []);

  async function inc_counter(projectId: string) {
    await invoke("inc_contribution", { projectId });
  }

  async function dec_counter(projectId: string) {
    await invoke("dec_contribution", { projectId });
  }

  async function create_projects() {
    await invoke("create_project", { color, title, reason });
    setOnCreateProject(false);
    resetFields();
  }

  useEffect(() => {
    get_projects()
    const unlisten = listen<string>("projects_updated", (_event) => {
      get_projects()
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  function get_color_from_project(id: string) {
    for (var index in projects) {
      if (projects[index].id == id) {
        return projects[index].color;
      }
    }
    return "#000000";
  }

  if (loading) return <div>Loading...</div>;

  return (
    <main>
      <div>
        <LineChart width={600} height={300} data={chartData}>
          <CartesianGrid stroke="#cccccc" />
          <XAxis dataKey="date" />
          <YAxis />
          {projectKeys.map((key, _i) => (
            <Line
              style={{ width: '100%', maxWidth: '300px', maxHeight: '70vh', aspectRatio: 1 / 1.618 }}
              strokeWidth={hoveredProjectId === key ? 4 : 2}
              key={key}
              dataKey={key}
              stroke={get_color_from_project(key)}
            />
          ))}
        </LineChart>
      </div>

      <div className="flex flex-col p-4">
        {/* <button onClick={create_projects}> bla</button> */}
        <div className="flex flex-row gap-4">
          <div className="flex flex-wrap flex-row gap-4"> {
            projects.map((p) => (
              <div key={p.id} className="flex flex-col bg-white shadow-md rounded-md max-w-xs w-[16vw] gap-2"
                onMouseEnter={() => setHoveredProjectId(p.id)}
                onMouseLeave={() => setHoveredProjectId(null)} >
                <div
                  style={{ backgroundColor: p.color }}
                  className="flex flex-row w-full justify-between rounded-t-lg">
                  <span className="px-4 py-2 cursor-pointer text-white hover:bg-white hover:text-black rounded-tl-md" onClick={() => dec_counter(p.id)}>-</span>
                  <span className="px-4 py-2 cursor-pointer text-white hover:bg-white hover:text-black rounded-tr-md" onClick={() => inc_counter(p.id)}>+</span>
                </div>
                <div className="flex flex-col gap-2 px-2 pb-2">
                  <div className="text-lg w-full font-bold">{p.title}</div>
                  <div className="text-base w-full text-justify">{p.reason}</div>
                </div>
              </div>
            ))}
          </div>
          {
            onCreateProject ?


              <div className="flex flex-col bg-white shadow-md rounded-md max-w-xs w-[16vw] gap-2">

                <div style={{ backgroundColor: color }} className="flex flex-row w-full justify-between rounded-t-lg">
                  <input
                    type="color"
                    value={color}
                    onChange={(e) => setColor(e.target.value)}
                    className="w-full h-full p-0 border-0 outline-none cursor-pointer overflow-hidden opacity-0"
                  />
                  <span className="px-4 py-2 cursor-pointer text-white hover:bg-white hover:text-black rounded-tr-md"
                    onClick={() => {
                      setOnCreateProject(false);
                      resetFields();
                    }}>
                    x
                  </span>
                </div>
                <div className="flex flex-col gap-2 px-2 pb-2">
                  <input placeholder="Title..." onChange={(e) => setTitle(e.target.value)} />
                  <input placeholder="Reason..." onChange={(e) => setReason(e.target.value)} />
                </div>

                <button onClick={create_projects}>Create</button>
              </div>
              :
              <div className="ml-[3vw]">
                <button onClick={() => { setOnCreateProject(true) }}>+</button>
              </div>
          }
        </div>
      </div>
    </main>
  );
}

export default App;
