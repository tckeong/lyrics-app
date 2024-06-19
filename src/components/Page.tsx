import { useNavigate } from "react-router-dom";

interface PageProps {
    name: string;
}

const Page = ({name}: PageProps) => {
    const navigate = useNavigate();
    
    return (
        <div>
        <h1 className="text-3xl font-bold underline">Page + {name}</h1>
        <button onClick={() => navigate("/")}>Hello</button>
        <button onClick={() => navigate("/page")}>Page</button>
        </div>
    );
}

export default Page;